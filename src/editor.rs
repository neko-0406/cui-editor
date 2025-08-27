use ratatui::widgets::ScrollbarState;

// エディター操作時のモード
#[derive(Clone, Copy, PartialEq)]
pub enum EditMode {
    Write,
    View,
}

#[derive(Clone)]
pub struct Editor {
    // 表示する内容
    pub content: String,
    // スクロールバー関係
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
    pub edit_mode: EditMode
}

// 内容関係の処理
impl Editor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            vertical_scroll_state: ScrollbarState::default(),
            horizontal_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
            horizontal_scroll: 0,
            edit_mode: EditMode::View
        }
    }

    pub fn get_content(&self) -> &str {
        return &self.content;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content
    }
}

// 操作関係の処理
impl Editor {
    fn scroll_down(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_add(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    fn scroll_up(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    fn scroll_left(&mut self) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
        self.horizontal_scroll_state = self.horizontal_scroll_state.position(self.horizontal_scroll);
    }

    fn scroll_right(&mut self) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);
        self.horizontal_scroll_state = self.horizontal_scroll_state.position(self.horizontal_scroll);
    }
}