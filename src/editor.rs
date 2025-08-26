use ratatui::widgets::ScrollbarState;

// エディター操作時のモード
#[derive(Clone, Copy, PartialEq)]
pub enum EditMode {
    Write,
    View,
}

pub struct Editor {
    // 表示する内容
    pub content: String,
    // モード
    pub edit_mode: EditMode,
    // スクロールバー関係
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize
}

// 内容関係の処理
impl Editor {
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

        // エディターのモード変更
    fn change_edit_mode(&mut self) {
        match self.edit_mode {
            EditMode::View => self.edit_mode = EditMode::Write,
            EditMode::Write => self.edit_mode = EditMode::View
        }
    }
}