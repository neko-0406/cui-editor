use std::{cell::RefCell, env::current_dir, io::{self, Error}};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, symbols::border, text::{Line, Text}, widgets::{Block, List, ListState, Paragraph, StatefulWidget, Widget}, DefaultTerminal, Frame};

mod file_manager;
use file_manager::FileItem;
// use setting;
pub struct CuiEditor {
    pub file_manager_width: u16,
    pub file_manage_path: String,
    pub file_item: Option<FileItem>,
    pub file_contents: String,
    pub file_item_str: Vec<String>,
    pub exit: bool,
    pub tree_state: RefCell<ListState>,
    pub app_focus: AppFocus,
    pub edit_mode: EditMode
}

// フォーカス制御用の列挙
#[derive(Clone, Copy, PartialEq)]
pub enum AppFocus {
    FileManager,
    Editor,
}

// エディター操作時のモード
#[derive(Clone, Copy, PartialEq)]
pub enum EditMode {
    Write,
    View,
}
// 実行、描画、イベントハンドル
impl CuiEditor {
    // アプリの変数初期化
    pub fn new() -> Result<Self, Error> {
        let mut app = Self {
            file_manager_width: 20,
            file_manage_path: String::new(),
            file_item: None,
            file_item_str: Vec::new(),
            file_contents: String::new(),
            exit: false,
            tree_state: RefCell::new(ListState::default()),
            app_focus: AppFocus::FileManager,
            edit_mode: EditMode::View
        };
        // 呼び出された現在のフォルダを開く
        let current_dir = current_dir()?;
        let root_item = FileItem::read_tree(current_dir)?;
        app.file_item = Some(root_item);
        Ok(app)
    }
    // メインプロセスの実行
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    // UIの描画
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    // イベントハンドリング
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    // キーイベントハンドリング
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        // ファイルマネージャー
        match (self.app_focus, key_event.modifiers, key_event.code) {
            (AppFocus::FileManager, KeyModifiers::NONE, KeyCode::Down) => self.select_next(),
            (AppFocus::FileManager, KeyModifiers::NONE, KeyCode::Up) => self.select_previous(),
            (AppFocus::FileManager, KeyModifiers::CONTROL, KeyCode::Up) => self.select_first(),
            (AppFocus::FileManager, KeyModifiers::CONTROL, KeyCode::Down) => self.select_last(),
            (AppFocus::FileManager, KeyModifiers::NONE, KeyCode::Esc) => self.select_none(),
            (AppFocus::FileManager, KeyModifiers::SHIFT, KeyCode::Right) => self.change_large(),
            (AppFocus::FileManager, KeyModifiers::SHIFT, KeyCode::Left) => self.change_small(),
            (AppFocus::FileManager, KeyModifiers::NONE, KeyCode::Enter) => {
                self.toggle_selected_directory();
                self.selected_file_display();
            }
            _ => {}
        }
        // エディター
        match (self.app_focus, key_event.modifiers, key_event.code ){
            (AppFocus::Editor, KeyModifiers::ALT, KeyCode::Char('c')) => self.change_edit_mode(),
            _ => {}
        }
        
        // グローバル
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL ,KeyCode::Char('q')) => self.exit(),
            (KeyModifiers::ALT, KeyCode::Char('e')) => self.focus_editor(),
            (KeyModifiers::ALT, KeyCode::Char('f')) => self.focus_filemanager(),
            _ => {}
        }
    }

    // ファイルマネージャーの操作
    // 選択解除
    fn select_none(&mut self) {
        self.tree_state.borrow_mut().select(None);
    }
    // 1個後へ
    fn select_next(&mut self) {
        self.tree_state.borrow_mut().select_next();
    }
    // 1個前へ
    fn select_previous(&mut self) {
        self.tree_state.borrow_mut().select_previous();
    }
    // 最初の場所へ
    fn select_first(&mut self) {
        self.tree_state.borrow_mut().select_first();
    }
    // 最後の場所へ
    fn select_last(&mut self) {
        self.tree_state.borrow_mut().select_last();
    }
    // 表示枠を大きく
    fn change_large(&mut self) {
        self.file_manager_width += 5;
    }
    // 表示枠を小さく
    fn change_small(&mut self) {
        self.file_manager_width -= 5;
    }

    // 現在選択されているFileItemを取得する
    fn get_selected_item(&self) -> Option<&FileItem> {
        let selected_index = self.tree_state.borrow().selected();
        if let (Some(index), Some(root_item)) = (selected_index, self.file_item.as_ref()) {
            return root_item.get_item_from_flat_index(index);
        }
        None
    }

    // 選択されているディレクトリの開閉を切り替える
    fn toggle_selected_directory(&mut self) {
        let selected_path = self
            .get_selected_item()
            .map(|item| item.get_path().to_path_buf());

        if let Some(path) = selected_path {
            if let Some(root_item) = self.file_item.as_mut() {
                if let Some(item_to_toggle) = root_item.find_item_by_path_mut(&path) {
                    item_to_toggle.toggle_open();
                }
            }
        }
    }

    // 選択されたファイルの中身を右のエリアに表示する
    fn selected_file_display(&mut self) {
        let selected_path = self.get_selected_item();
        if let Some(item) = selected_path {
            if item.get_path().is_file() {
                if let  Some(content) = item.read_file() {
                    self.file_contents = content;
                    self.focus_editor();
                }
            }
        }
    }

    // エディターのモード変更
    fn change_edit_mode(&mut self) {
        match self.edit_mode {
            EditMode::View => self.edit_mode = EditMode::Write,
            EditMode::Write => self.edit_mode = EditMode::View
        }
    }
}

// アプリの状態変更用の関数
impl CuiEditor {
    // 終了フラグ
    fn exit(&mut self) {
        self.exit = true;
    }
    // フォーカスの変更
    fn focus_filemanager(&mut self) {
        if self.app_focus == AppFocus::Editor {
            self.app_focus = AppFocus::FileManager;
        }
    }
    fn focus_editor(&mut self) {
        if self.app_focus == AppFocus::FileManager {
            self.app_focus = AppFocus::Editor
        }
    }

}

// 描画用の処理
impl Widget for &CuiEditor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(self.file_manager_width), Constraint::Percentage(100-self.file_manager_width)])
            .split(area);

        // 左側のエリア
        // 現在のディレクトリツリーを表示
        if let Some(root_item) = self.file_item.as_ref() {
            let left_block = Block::bordered()
                .title(Line::from(root_item.get_name()).centered())
                .border_set(border::THICK);

            let tree_items = root_item.tree_to_string_without_root();
            let tree = List::new(tree_items)
                .block(left_block)
                .highlight_style(Style::default().bg(Color::Cyan).add_modifier(Modifier::BOLD));

            StatefulWidget::render(tree, layout[0], buf, &mut *self.tree_state.borrow_mut());
        }

        // 右側のエリア
        let right_block = Block::bordered()
            .border_set(border::THICK);

        // let content = &self.file_contents;
        // let mut text = Text::from("");
        // if let Some(result) = content {
        //     text = Text::from(result.clone());
        // }

        Paragraph::new(Text::from(self.file_contents.clone()))
            .block(right_block)
            .render(layout[1], buf);
    }
}