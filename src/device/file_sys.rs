use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub struct FileNode {
    file_name: String,
    is_file: bool,
    children: Vec<Rc<RefCell<FileNode>>>,
}

impl FileNode {
    fn new(file_name: String, is_file: bool) -> Self {
        FileNode {
            file_name,
            is_file,
            children: Vec::new(),
        }
    }

    fn add_child(&self, child: FileNode) {}
}

pub struct FileSystem<'a> {
    pub root_path: &'a Path,
    pub os: String,
    pub node: FileNode,
}

impl<'a> FileSystem<'a> {
    pub fn new(root_path: &'a Path, os: String) -> Self {
        FileSystem {
            root_path,
            os,
            node: FileNode::new(root_path.to_string_lossy().to_string(), root_path.is_file()),
        }
    }

    fn iter_sub_tree() {}

    pub fn get_tree_json() {}
}
