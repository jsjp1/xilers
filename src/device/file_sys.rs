use std::collections::VecDeque;
use std::path::{Path, PathBuf};

#[derive(Clone)]
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
                children: Vec::<FileNode>::new(),
            })
        } else {
            Some(FileNode {
                file_name: file_name.to_string(),
                children: Vec::<FileNode>::new(),
            })
        }
    }

    fn is_exist(path_str: &str) -> bool {
        let _path = Path::new(path_str);
        _path.exists()
    }
}

pub struct FileSystem {
    pub node: FileNode, // 결정된 root node
}

impl FileSystem {
    pub fn new(root_path: &mut str) -> Self {
        let file_node = FileNode::new(root_path, true).expect(&format!(
            "파일시스템 생성시 문제가 발생했습니다. root_path({:?})를 참고해주십시오.",
            root_path
        ));

        FileSystem { node: file_node }
    }

    pub fn init_file_node(&self) {
        // root path를 기준으로 leaf node file 나올때까지 child vector 채움
        let mut bfs_queue: VecDeque<(String, FileNode)> = VecDeque::new();
        bfs_queue.push_back((self.node.file_name.clone(), self.node.clone())); // file_name은 root path부터 현재 path까지 계속 이어짐 (e.g. / + dir1/ + dir2/ + file.txt)

        while let Some(_node) = bfs_queue.pop_front() {
            let _file_name_cont = _node.0; // root path만 absolute path, 나머지는 file name만 존재
            let _file_node = _node.1;

            let mut _node_child = _file_node.children;

            log::debug!("전체 경로: {:?}", _file_name_cont);
            let entries = std::fs::read_dir(&_file_name_cont).expect("경로가 적절하지 않습니다.");
            for _entry in entries {
                // loop 돌면서 queue와 child를 채움
                let entry = _entry.unwrap();
                let file_type = entry.file_type().unwrap();
                let file_name = entry.file_name();
                log::debug!("   현재 처리 경로: {:?}", file_name);

                if file_type.is_dir() {
                    // 디렉토리일 경우 queue에 푸시하고 child node에 추가
                    let mut _dir_str = PathBuf::from(&_file_name_cont);
                    _dir_str.push(&file_name);
                    let new_node = FileNode::new(&file_name.to_str().unwrap(), false);

                    match new_node {
                        Some(node) => {
                            bfs_queue
                                .push_back((_dir_str.to_str().unwrap().to_owned(), node.clone()));
                            _node_child.push(node);
                        }
                        None => continue,
                    }
                } else if file_type.is_file() {
                    // 파일일 경우 child node에 추가
                    let new_node = FileNode::new(&file_name.to_str().unwrap(), false);

                    match new_node {
                        Some(node) => {
                            _node_child.push(node);
                        }
                        None => continue,
                    }
                } else {
                    // TODO: symlink 같은 경우 추가?
                    log::warn!(
                        "지원하지 않는 파일 타입({:?})입니다. {:?}",
                        file_type,
                        file_name
                    );
                    continue;
                }
            }
        }
    }
}
