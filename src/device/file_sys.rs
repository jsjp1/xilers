use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, BorrowMut};

use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Deserialize)]
pub struct FileNode {
    file_name: String,
    children: Vec<FileNode>,
}

impl std::fmt::Debug for FileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut deque: VecDeque<(FileNode, usize, usize)> = VecDeque::new();
        deque.push_front((self.clone(), 0, 0));
        let _ = write!(f, "\n");

        while deque.is_empty() != true {
            let opt = deque.pop_back().unwrap();
            let file_node = opt.0;
            let indent = opt.1;
            let child = file_node.children;

            let formatted_str: String;
            if child.len() >= 1 {
                formatted_str =
                    format!("{}{}/\n", " ".repeat((indent + 1) * 4), file_node.file_name);
            } else {
                formatted_str =
                    format!("{}{}\n", " ".repeat((indent + 1) * 4), file_node.file_name);
            }

            let _ = write!(f, "{}", formatted_str);

            for (idx, node) in child.iter().enumerate() {
                if idx >= 7 {
                    // 숫자 변경 필요
                    break;
                }
                deque.push_back((node.clone(), indent + 1, idx));
            }
        }

        Ok(())
    }
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

#[derive(Clone, Serialize, Deserialize)]
pub struct FileSystem {
    pub node: FileNode, // 결정된 root node
}

impl std::fmt::Debug for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.node)
    }
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
