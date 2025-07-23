use std::io::{self};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::{self, Constraint, Direction, Layout}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal, Frame};

#[derive(Default)]
struct Task {
    id: String,
    time_stamp: String,
    title: String,
    description: String,
    completed: bool,
}

#[derive(Default)]
struct ToDoApp {
    tasks: Vec<Task>,
    exit: bool,
}

// 実行、描画、イベントハンドル
impl ToDoApp {
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
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
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
    // タスクの追加
    fn add_task(&mut self, task:Task) {
        self.tasks.push(task);
    }
    // タスクの永続保存
    fn save_tasks(&mut self) {}
}

// 描画用の処理
impl Widget for &ToDoApp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        // let title = Line::from("Todo Manage App".bold());
        // let block = Block::bordered()
        //     .title(title.centered())
        //     .border_set(border::THICK);

        // Paragraph::new(Text::from("test!!")).block(block).render(area, buf);
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        // 左側のエリア
        let left_block = Block::bordered()
            .title(Line::from("Files").centered())
            .border_set(border::THICK);

        Paragraph::new(Text::from("left"))
            .block(left_block)
            .render(layout[0], buf);

        // 右側のエリア
        let right_block = Block::bordered()
            .title(Line::from("right").centered())
            .border_set(border::THICK);

        Paragraph::new(Text::from("right"))
            .block(right_block)
            .render(layout[1], buf);
    }
}

fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();
    let app_result = ToDoApp::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
