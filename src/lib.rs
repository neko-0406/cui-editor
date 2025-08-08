use std::{cell::RefCell, env::current_dir, io::{self, Error}};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, symbols::border, text::{Line, Text}, widgets::{Block, Borders, List, ListState, Paragraph, StatefulWidget, Widget}, DefaultTerminal, Frame};

mod file_manager;
use file_manager::FileItem;
// use setting;
pub struct ToDoApp {
    pub file_manager_width: u16,
    pub file_item: Option<FileItem>,
    pub exit: bool,
    pub tree_state: RefCell<ListState>,
    pub app_focus: AppFocus
}

// フォーカス制御用の列挙
#[derive(Clone, Copy)]
pub enum AppFocus {
    FileManager,
    Editor,
}

// 実行、描画、イベントハンドル
impl ToDoApp {
    // アプリの変数初期化
    pub fn new() -> Result<Self, Error> {
        let mut app = Self {
            file_manager_width: 20,
            file_item: None,
            exit: false,
            tree_state: RefCell::new(ListState::default()),
            app_focus: AppFocus::FileManager
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
            _ => {}
        }
        
        // グローバル
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL ,KeyCode::Char('q')) => self.exit(),
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

}

// アプリの状態変更用の関数
impl ToDoApp {
    // 終了フラグ
    fn exit(&mut self) {
        self.exit = true;
    }
}

// 描画用の処理
impl Widget for & ToDoApp {
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

            let tree_items: Vec<String> = root_item.tree_to_string_without_root();
            let tree = List::new(tree_items)
                .block(left_block)
                .highlight_style(Style::default().bg(Color::Cyan).add_modifier(Modifier::BOLD));

            StatefulWidget::render(tree, layout[0], buf, &mut *self.tree_state.borrow_mut());
        }

        // 右側のエリア
        let right_block = Block::bordered()
            .border_set(border::THICK);

        Paragraph::new(Text::from("test"))
            .block(right_block)
            .render(layout[1], buf);
    }
}