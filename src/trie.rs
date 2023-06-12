use std::{rc::Rc, cell::RefCell, collections::HashMap};

#[derive(Debug)]
pub struct Node {
    is_leaf: bool,
    links: HashMap<String, Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Self {
        Node { is_leaf: false, links: HashMap::new() }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: Rc<RefCell<Node>>,
    pub size: usize,
}

impl Trie {
    pub fn new() -> Self {
        return Trie {
            root: Rc::new(RefCell::new(Node::new())),
            size: 0,
        };
    }

    pub fn insert(&mut self, postfix: String) {
        let mut tmp = self.root.clone();

        for post in  postfix.split('\\') {
            // exist postfix continue back_insert
            let may_exist = tmp.borrow().links.get(post).cloned();
            if let Some(exist) = may_exist {
                tmp = exist.to_owned().clone();

            // link new postfix
            } else {
                let node = Rc::new(RefCell::new(Node::new()));
                tmp.borrow_mut().links.insert(post.to_owned(), node.clone());
                tmp = node;
            };
        }
        if !tmp.borrow().is_leaf  {
            tmp.borrow_mut().is_leaf = true;
            self.size += 1;
        }
    }

    pub fn search(&self,  postfix: String) -> bool {
        let mut tmp = self.root.clone();

        for post in  postfix.split('\\') {
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

        for post in  postfix.split('\\') {
            let may_exist = tmp.borrow().links.get(post).cloned();
            if let Some(exist) = may_exist {
                tmp = exist.clone();
            } else {
                return false;
            };
        }
        return true;
    }
}
