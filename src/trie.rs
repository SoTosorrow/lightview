use std::{rc::Rc, cell::RefCell};
use crate::node::*;


#[derive(Debug)]
pub struct Trie {
    root: Rc<RefCell<Node>>,
    pub size: usize,
}

impl Trie {
    pub fn new() -> Self {
        let node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().is_dir = true;
        Trie {
            root: node,
            size: 0,
        }
    }

    pub fn insert(&mut self, postfix: String, is_dir: bool) {
        let mut tmp = self.root.clone();

        for post in postfix.split('\\') {
            // exist postfix continue back_insert
            let may_exist = tmp.borrow().links.get(post).cloned();
            if let Some(exist) = may_exist {
                tmp = exist.to_owned().clone();

            // link new postfix
            } else {
                let node = Rc::new(RefCell::new(Node::new()));
                node.borrow_mut().path = postfix.clone();
                tmp.borrow_mut().links.insert(post.to_owned(), node.clone());
                tmp = node;
            };
        }
        if !tmp.borrow().is_leaf  {
            tmp.borrow_mut().is_leaf = true;
            tmp.borrow_mut().is_dir = is_dir;
            self.size += 1;
        }
    }

    // pub fn ignore(&mut self, postfix: String) {

    // }

    pub fn search(&self,  postfix: String) -> bool {
        let mut tmp = self.root.clone();

        for post in postfix.split('\\') {
            let may_exist = tmp.borrow().links.get(post).cloned();
            if let Some(exist) = may_exist {
                tmp = exist.clone();
            } else {
                return false;
            };
        }
        if tmp.borrow().is_leaf {
            return true;
        }
        return false;
    }

    pub fn start_with(&self,  postfix: String) -> bool {
        let mut tmp = self.root.clone();

        for post in postfix.split('\\') {
            let may_exist = tmp.borrow().links.get(post).cloned();
            if let Some(exist) = may_exist {
                tmp = exist.clone();
            } else {
                return false;
            };
        }
        return true;
    }

    pub fn collect(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut v: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(self.size);
        let mut queue: Vec<Rc<RefCell<Node>>> = Vec::new();

        let node = self.root.clone();
        queue.push(node.clone());
        
        while queue.len() > 0 {
            let node = queue.pop().unwrap();
            // v.push(node.clone());
            node.borrow().links.iter().for_each(|i| {
                if !i.1.borrow().is_leaf {
                    queue.push(i.1.clone());
                } else {
                    if i.1.borrow().is_dir {
                        queue.push(i.1.clone());
                    }
                    v.push(i.1.clone());
                }
            });
        }
        v

    }
}
