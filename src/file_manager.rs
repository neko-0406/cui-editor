use std::{io, path::{Path, PathBuf}};

#[derive(Default)]
pub struct FileItem {
    name: String,
    path: PathBuf,
    items: Option<Vec<Self>>,
    is_open: Option<bool>
}

impl FileItem {
    // ファイルアイテムの生成
    pub fn new(item_path: &Path) -> Self {
        let os_file_name = item_path.file_name().unwrap();
        let file_name = os_file_name.to_str().unwrap();
        // ファイルならリストなし、フォルダならあり
        if item_path.is_dir() {
            Self {
                name: file_name.to_owned(),
                path: item_path.to_path_buf(),
                items: Some(Vec::new()),
                is_open: Some(true)
            }
        } else {
            Self {
                name: file_name.to_owned(),
                path: item_path.to_path_buf(),
                items: None,
                is_open: None
            }
        }
    }
    // ファイルアイテムの名前取得
    pub fn get_name(&self) -> &str {
        &self.name
    }
    // ファイルパスの取得
    pub fn get_path(&self) -> &Path {
        &self.path
    }
    // アイテムの子リストを取得
    pub fn get_items(&self) -> Option<&Vec<Self>> {
        self.items.as_ref()
    }
    // 子リストにアイテムを追加
    pub fn add_item(&mut self, item: FileItem) {
        if let Some(items) = &mut self.items {
            items.push(item);
        }
    }
    // フォルダの状態を取得
    pub fn get__dir_opened(&self) -> Option<bool> {
        if let Some(opened) = self.is_open {
            return Some(opened)
        }
        None
    }
    // 再帰的にツリーの構築
    pub fn read_tree(root_path: PathBuf) -> Result<FileItem, io::Error> {
        if root_path.is_file() {
            return Ok(FileItem::new(&root_path));
        } else {
            let mut file_item = FileItem::new(&root_path);
            let entries = root_path.read_dir()?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    file_item.add_item(FileItem::new(&path));
                } else {
                    let child_tree = FileItem::read_tree(path)?;
                    file_item.add_item(child_tree);
                }
            }
            return Ok(file_item);
        }
    }
    // ツリーを文字列のリストとして表現
    pub fn tree_to_string(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        self.flatten_tree(&mut result, 0);
        result
    }

    fn flatten_tree(&self, result: &mut Vec<String>, level: usize) {
        // インデントの作成
        let indent = " ".repeat(level);
        // アイコンの選定
        let icon = if self.items.is_some() {
            if self.is_open.unwrap_or(false) {"▶"} else {"▼"}
        } else {
            ""
        };
        // 今のアイテムを追加
        result.push(format!("{}{} {}", indent, icon, self.name));

        // 子アイテムがあって、開いてたら再帰処理
        if let Some(items) = &self.items {
            if self.is_open.unwrap_or(false) {
                for item in items {
                    item.flatten_tree(result, level + 1);
                }
            }
        }
    }
}

