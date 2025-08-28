use crate::editor::Editor;

#[derive(Clone)]
pub struct Tab {
    pub title: String,
    pub content: String,
    pub editor: Editor
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

    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }
    
    pub fn get_tabs(&self) -> &Vec<Tab> {
        return self.tabs.as_ref();
    }
}

