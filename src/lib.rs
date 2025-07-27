use std::{io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{layout::{Constraint, Direction, Layout}, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal, Frame};

#[derive(Default)]
pub struct Task {
    pub id: String,
    pub time_stamp: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub group_id: String
}

#[derive(Default)]
pub struct TaskGroup {
    pub id: String,
    pub title: String
}

#[derive(Default)]
pub struct ToDoApp {
    pub tasks: Vec<Task>,
    pub task_group: Vec<TaskGroup>,
    pub exit: bool,
    pub selected_group_id: String,
    pub selected_task_id: String
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
    // タスクの追加
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    // タスクの永続保存
    pub fn save_tasks(&mut self) {}
}

// 描画用の処理
impl Widget for &ToDoApp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        // 左側のエリア
        let left_block = Block::bordered()
            .title(Line::from("Group").centered())
            .border_set(border::THICK);

        Paragraph::new(Text::from("group1"))
            .block(left_block)
            .render(layout[0], buf);

        // 右側のエリア
        let right_block = Block::bordered()
            .title(Line::from("Tasks").centered())
            .border_set(border::THICK);

        Paragraph::new(Text::from("task1"))
            .block(right_block)
            .render(layout[1], buf);
    }
}