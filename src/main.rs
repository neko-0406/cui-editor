use std::io;
use todo_manager::ToDoApp;

fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();
    let app_result = ToDoApp::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
