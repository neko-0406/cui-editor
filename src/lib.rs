use std::{env::current_dir, io::{self, Error}};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{layout::{Constraint, Direction, Layout}, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal, Frame};

mod file_manager;
use file_manager::FileItem;
// use setting;

#[derive(Default)]
pub struct ToDoApp {
    pub file_manager_width: u16,
    pub file_item: Option<FileItem>,
    pub exit: bool,
}

// 実行、描画、イベントハンドル
impl ToDoApp {
    // アプリの変数初期化
    pub fn new() -> Result<Self, Error> {
        let mut app = Self { file_manager_width: 20, file_item: None, exit: false };
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
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL ,KeyCode::Char('q')) => self.exit(),
            _ => {}
        }
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
impl Widget for &ToDoApp {
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

            

            // Paragraph::new(Text::from("選択されていません"))
            //     .block(left_block)
            //     .render(layout[0], buf);
        }

        // 右側のエリア
        let right_block = Block::bordered()
            .border_set(border::THICK);

        Paragraph::new(Text::from("test"))
            .block(right_block)
            .render(layout[1], buf);
    }
}