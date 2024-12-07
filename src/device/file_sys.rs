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
            // TODO: fs::metadata 이용해서 변경하기
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
        // 운영체제에 따라 기본 root_path 설정
        if FileNode::is_exist(root_path) != true {
            println!(
                "root_path: {:?}가 존재하지 않습니다. 운영체제별 Default path로 설정합니다.",
                root_path
            );
            let default_root_path = match std::env::consts::OS {
                "windows" => "C:\\Users\\Public",
                "macos" => "/Users/Shared",
                "linux" => "/home",
                "android" => "/sdcard",
                "ios" => "/var/mobile",
                _ => panic!("지원하지 않는 운영체제입니다. 프로그램을 종료합니다."),
            };

            let file_node = FileNode::new(default_root_path, true).expect(&format!(
                "파일시스템 생성시 문제가 발생했습니다. root_path({:?})를 참고해주십시오.",
                default_root_path
            ));

            FileSystem { node: file_node }
        } else {
            let file_node = FileNode::new(root_path, true).expect(&format!(
                "파일시스템 생성시 문제가 발생했습니다. root_path({:?})를 참고해주십시오.",
                root_path
            ));
            FileSystem { node: file_node }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    fn setup_test_directory() -> &'static str {
        let test_dir = "test_root";
        fs::create_dir_all(test_dir).unwrap();
        let _ = File::create(format!("{}/file1.txt", test_dir)).unwrap();
        let _ = File::create(format!("{}/file2.txt", test_dir)).unwrap();
        fs::create_dir_all(format!("{}/subdir", test_dir)).unwrap();
        let _ = File::create(format!("{}/subdir/file3.txt", test_dir)).unwrap();
        test_dir
    }

    #[test]
    fn test_file_node_new_valid() {
        let test_dir = setup_test_directory();

        let node = FileNode::new(test_dir, true);
        assert!(node.is_some());
    }

    #[test]
    fn test_file_node_new_invalid() {
        let node = FileNode::new("invalid_path", true);
        assert!(node.is_none());
    }

    #[test]
    fn test_file_system_new_with_existing_root() {
        let test_dir = setup_test_directory();

        let fs = FileSystem::new(&mut test_dir.to_string());
        println!("{}", fs.node.file_name);
        assert_eq!(fs.node.file_name, "test_root");
    }

    #[test]
    fn test_file_system_init_file_node() {
        let test_dir = setup_test_directory();

        let mut fs = FileSystem::new(&mut test_dir.to_string());
        fs.init_file_node();

        assert!(fs.node.children.len() > 0); // Root should have children
        assert_eq!(fs.node.children[0].file_name, "file2.txt");
    }
}
