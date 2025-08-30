use std::{fs::File, io::{Read}, path::{Path, PathBuf}};

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
    pub file_path: PathBuf,
    // スクロールバー関係
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
    pub edit_mode: EditMode
}

// 内容関係の処理
impl Editor {
    // 初期化処理
    pub fn new(file_path: &Path) -> Self {
        let pathbuf = file_path.to_path_buf();
        let mut content = String::new();
        let vertical_scroll_state = ScrollbarState::default();
        let horizontal_scroll_state = ScrollbarState::default();
        let vertical_scroll: usize = 0;
        let horizontal_scroll: usize = 0;

        Editor::read_file(&pathbuf, &mut content);

        return Self {
            file_path: pathbuf,
            content: content,
            vertical_scroll_state: vertical_scroll_state,
            horizontal_scroll_state: horizontal_scroll_state,
            vertical_scroll: vertical_scroll,
            horizontal_scroll: horizontal_scroll,
            edit_mode: EditMode::View
        };
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

// IO関係の処理
impl Editor {
    //ファイルパスから中身の取得
    pub fn read_file(path: &Path, content: &mut String) {
        let mut file = File::open(path).expect("Failed to open file...");
        file.read_to_string(content)
            .expect("Failed to open file...");
    }
}