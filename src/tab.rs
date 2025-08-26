use crate::editor::{Editor};

pub struct Tab {
    pub title: String,
    pub editor: Editor
}

impl Tab {
    fn new(title: &str, editor: Editor) -> Self {
        Self { title: title.to_owned(), editor: editor }
    }
}

pub struct TabContainer {
    children: Vec<Tab>,
    tab_labels: Vec<String>,
    index: usize,
}

impl TabContainer {
    pub fn new() -> Self {
        Self { children: Vec::new(), tab_labels: Vec::new(), index: 0 }
    }

    pub fn push_tab(&mut self, tab: Tab) {
        self.children.push(tab);
        self.update_tab_label();
    }

    pub fn remove_tab(&mut self, index: usize) {
        self.children.remove(index);
        self.update_tab_label();
    }

    pub fn remove_all_tab (&mut self) {
        self.children.clear();
        self.update_tab_label();
    }

    fn update_tab_label(&mut self) {
        self.tab_labels = self.children.iter()
                .map(|tab| tab.title.clone())
                .collect();
    }
}