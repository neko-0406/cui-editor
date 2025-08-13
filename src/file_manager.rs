use std::{io, path::{Path, PathBuf}};

#[derive(Default, Clone)]
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
                is_open: Some(false)
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
    pub fn _get__dir_opened(&self) -> Option<bool> {
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

    // ルートフォルダを表示せずに、その中身だけを表示
    pub fn tree_to_string_without_root(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        
        // ルートの子要素があれば、それらを直接表示
        if let Some(items) = &self.items {
            for item in items {
                item.flatten_tree_str(&mut result, 0); // レベル0から開始
            }
        }
        
        result
    }

    fn flatten_tree_str(&self, result: &mut Vec<String>, level: usize) {
        // インデントの作成
        let indent = " ".repeat(level);
        // アイコンの選定
        let icon = if self.items.is_some() {
            if self.is_open.unwrap_or(false) {"📂"} else {"📁"}
        } else {
            "📄"
        };
        // 今のアイテムを追加
        result.push(format!("{}{} {}", indent, icon, self.name));

        // 子アイテムがあって、開いてたら再帰処理
        if let Some(items) = &self.items {
            if self.is_open.unwrap_or(false) {
                for item in items {
                    item.flatten_tree_str(result, level + 1);
                }
            }
        }
    }

    // 可視ツリーをフラットなVecにして、そこから選択されたFileItemを返す
    pub fn get_item_from_flat_index(&self, index: usize) -> Option<&Self> {
        let mut flat_list = Vec::new();
        if let Some(items) = &self.items {
            for item in items {
                item.flatten_to_vec(&mut flat_list);
            }
        }
        flat_list.get(index).copied()
    }

    // ツリーをフラットなVec<&FileItem>に変換する再帰関数
    fn flatten_to_vec<'a>(&'a self, vec: &mut Vec<&'a Self>) {
        vec.push(self);
        // 子アイテムがあって、開いてたら再帰処理
        if self.is_open.unwrap_or(false) {
            if let Some(items) = &self.items {
                for item in items {
                    item.flatten_to_vec(vec);
                }
            }
        }
    }

    // パスをもとに可変のFileItemを探す
    pub fn find_item_by_path_mut<'a>(&'a mut self, path: &std::path::Path) -> Option<&'a mut Self> {
        if self.path == path {
            return Some(self);
        }
        if let Some(items) = &mut self.items {
            for item in items {
                if let Some(found) = item.find_item_by_path_mut(path) {
                    return Some(found);
                }
            }
        }
        None
    }

    // ディレクトリの開閉状態を切り替える
    pub fn toggle_open(&mut self) {
        if self.items.is_some() { // ディレクトリかどうか
            if let Some(is_open) = self.is_open.as_mut() {
                *is_open = !*is_open;
            }
        }
    }
}

