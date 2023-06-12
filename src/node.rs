use crate::pair::*;

pub struct LNode<'a> {
    links:      Vec<LNode<'a>>,
    pair:       Option<Pair<&'a str>>,
    is_dir:     bool,
    is_ignore:  bool,           // 忽略文件
    // node_name: &'a str,
    // suffix: String,
}

impl<'a> LNode<'a> {
    pub fn new() -> Self{
        LNode { 
            links: Vec::new(), 
            pair: None, 
            is_dir: false, 
            is_ignore: false 
        }
    }

    pub fn set_pair(&mut self, pair: Pair<&'a str>) {
        self.pair = Some(pair);
    }
}

// struct LTree {
//     node: Box<LTree>,
// }