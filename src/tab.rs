use crate::editor::Editor;

#[derive(Clone)]
pub struct Tab {
    title: String,
    content: String,
    editor: Editor
}

impl Tab {
    pub fn new(title: &str, content: &str) -> Self {
        Self { title: title.to_owned(), content: content.to_owned(), editor: Editor::new() }
    }
}

pub struct TabContainer {
    tabs: Vec<Tab>,
    selected_index: usize
}

impl TabContainer {
    pub fn new() -> Self {
        Self { tabs: Vec::new(), selected_index: 0 }
    }
    
}

