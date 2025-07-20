use std::io::{self, stdout};

use crossterm::{execute, terminal::{enable_raw_mode, EnterAlternateScreen}};
use ratatui::{prelude::CrosstermBackend, restore, Terminal};

// ターミナルの初期化
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    set_panic_hook();
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore();
        hook(panic_info);
    }));
}