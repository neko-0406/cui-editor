use std::{io::{self, stdout}, time::Duration};

use crossterm::{event::{self, Event, KeyCode, KeyEventKind}, terminal::disable_raw_mode};
use ratatui::{crossterm::terminal::enable_raw_mode, prelude::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|_frame| {
            // ここにUIの描画ロジックを追加
            // 例: frame.render_widget(...);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}
