use std::io;
use cui_editor::ToDoApp;

fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();
    let app_result = ToDoApp::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
