use std::{io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{layout::{Constraint, Direction, Layout}, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal, Frame};

#[derive(Default)]
pub struct ToDoApp {
    pub file_manager_width: u16,
    pub open_folder_path: String,
    pub exit: bool,
}

// 実行、描画、イベントハンドル
impl ToDoApp {
    pub fn new() -> Self {
        Self { file_manager_width: 20, open_folder_path: String::from(""), exit: false }
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
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(self.file_manager_width), Constraint::Percentage(100-self.file_manager_width)])
            .split(area);

        // 左側のエリア
        let left_block = Block::bordered()
            .title(Line::from("File Manager").centered())
            .border_set(border::THICK);

        Paragraph::new(Text::from("ファイル1.md"))
            .block(left_block)
            .render(layout[0], buf);

        // 右側のエリア
        let right_block = Block::bordered()
            .border_set(border::THICK);

        Paragraph::new(Text::from("test"))
            .block(right_block)
            .render(layout[1], buf);
    }
}