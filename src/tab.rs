use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Paragraph, Widget}};

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
    pub children: Vec<Tab>,
    pub tab_labels: Vec<String>,
    pub index: usize,
}

impl TabContainer {
    pub fn new() -> Self {
        Self { children: Vec::new(), tab_labels: Vec::new(), index: 0 }
    }

    // pub fn render_tab(&self, area: Rect, block: Block<'_>, buf: &mut Buffer) {
    //     if let Some(tab) = self.children.get(self.index) {
    //         Paragraph::new(tab.editor.content.clone())
    //             .block(block)
    //             .render(area, buf);
    //     }
    // }

    fn push_tab(&mut self, tab: Tab) {
        self.children.push(tab);
        self.update_tab_label();
    }

    fn remove_tab(&mut self, index: usize) {
        self.children.remove(index);
        self.update_tab_label();
    }

    fn remove_all_tab (&mut self) {
        self.children.clear();
        self.update_tab_label();
    }

    fn update_tab_label(&mut self) {
        self.tab_labels = self.children.iter()
                .map(|tab| tab.title.clone())
                .collect();
    }
}