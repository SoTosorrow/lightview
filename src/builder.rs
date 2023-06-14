use walkdir::WalkDir;
use rayon::prelude::*;

use crate::trie::Trie;

pub struct Builder {
    trie: Box<Trie>,
}

impl Builder {
    pub fn build(&mut self) -> &Self {
        self
    }

    pub fn build_with_dir(root_path: &str) -> Self {
        let mut trie = Box::new(Trie::new());

        for entry in WalkDir::new(&root_path) {
            let entry = entry.unwrap();
            let is_dir = entry.path().is_dir();
            let entry = entry.path().to_string_lossy().to_string();
            // let entry = entry.trim_start_matches(&root_path).to_string();
            trie.insert(entry, is_dir);
        }

        Builder { trie }
    }

    pub fn build_to_vec(root_path: String) {
        for entry in WalkDir::new(&root_path) {
            todo!()
        }
    }

    //@todo replace collect() to build_to_vec
    pub fn on_config(&self, config: &str) {
        let n = Trie::as_clone_vec(self.trie.collect());
        let c = n.par_iter().map(move |i| {
            // println!("{:?}",i.path);
            i
        }).count();
        println!("{}",c);
    }
}