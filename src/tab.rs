use std::path::Path;

use crate::editor::Editor;

#[derive(Clone)]
pub struct Tab {
    pub title: String,
    pub editor: Editor
}

impl Tab {
    pub fn new(title: &str, file_path: &Path) -> Self {
        Self { title: title.to_owned(), editor: Editor::new(file_path) }
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
    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_index = index;
    }
    
    pub fn get_tabs(&self) -> &Vec<Tab> {
        return self.tabs.as_ref();
    }

    pub fn push_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
    }
}

