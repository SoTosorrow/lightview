use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<'a> {
    pub path:       Option<String>,
    pub links:      Vec<Rc<RefCell<Node<'a>>>>,
    // pub path:       Option<&'a Path>,
    // pub pair:       Option<Pair<&'a str>>,
    pub is_dir:     bool,
    // pub is_ignore:  bool,           // 忽略文件
    // node_name: &'a str,
    // suffix: String,
}

impl<'a> Default for Node<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?})", self.path)
    }
}

impl<'a> Node<'a> {
    pub fn new() -> Self{
        Node { 
            links: Vec::new(), 
            path: None,
            // pair: None, 
            is_dir: false, 
            // is_ignore: false 
        }
    }
}

// struct LTree {
//     node: Box<LTree>,
// }