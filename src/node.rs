use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::pin::Pin;
use std::ptr;

#[derive(Debug)]
pub struct Node {
    pub path: String,
    // pub file_name: *const String,
    pub links: HashMap<String, Rc<RefCell<Node>>>,
    pub is_dir:     bool,
    pub is_ignore:  bool,           // 忽略文件
    pub is_leaf:    bool,
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new() -> Self{
        Node { 
            path: String::from(""),
            // file_name: ptr::null(),
            links: HashMap::new(), 
            is_dir: false, 
            is_ignore: false, 
            is_leaf: false,
        }
    }

    // pub fn set_file_name(self :Pin<&mut Self>) {
    //     let this : &mut Self = unsafe { self.get_unchecked_mut() };
    //     this.file_name = &mut this.path as *const String;
    // }
}