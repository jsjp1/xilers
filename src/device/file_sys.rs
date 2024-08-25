use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, BorrowMut};

use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileNode {
    file_name: String,
    // is_file: bool, file_name 맨 뒤 '/'가 있으면 디렉토리, 아니면 파일로 구분
    children: Vec<FileNode>,
}

impl FileNode {
    pub fn new(file_name: &str, is_root: bool) -> Option<Self> {
        if is_root {
            if FileNode::is_exist(file_name) == false {
                log::warn!(
                    "주어진 파일 혹은 경로({:?})가 존재하지 않습니다.",
                    file_name
                );

                return None;
            }

            Some(FileNode {
                file_name: file_name.to_string(),
                children: Vec::new(),
            })
        } else {
            Some(FileNode {
                file_name: file_name.to_string(),
                children: Vec::new(),
            })
        }
    }

    pub fn add_child(&mut self, child: FileNode) {
        self.children.push(child);
    }

    fn is_exist(path_str: &str) -> bool {
        let _path = Path::new(path_str);
        _path.exists()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileSystem {
    pub node: FileNode, // 결정된 root node
}

// TODO: unwrap 처리
impl FileSystem {
    pub fn new(root_path: &mut str) -> Self {
        let file_node = FileNode::new(root_path, true).expect(&format!(
            "파일시스템 생성시 문제가 발생했습니다. root_path({:?})를 참고해주십시오.",
            root_path
        ));

        FileSystem { node: file_node }
    }

    pub fn init_file_node(&mut self) {
        let file_name = self.node.file_name.clone();
        let root_path = Path::new(&file_name);
        FileSystem::build_tree(&mut self.node, root_path);
    }

    fn build_tree(current_node: &mut FileNode, path: &Path) {
        if path.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let file_name = entry.file_name().to_string_lossy().into_owned();

                        let mut child_node = FileNode::new(&file_name, false).unwrap();

                        FileSystem::build_tree(&mut child_node, &path);

                        current_node.add_child(child_node);
                    }
                }
            }
        }
    }
}
