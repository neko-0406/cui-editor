use std::io;
use cui_editor::CuiEditor;

fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();
    let app_result = CuiEditor::new().unwrap().run(&mut terminal);
    ratatui::restore();
    app_result
}
