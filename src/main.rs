use std::fs::read_dir;
use std::result::Result;
use std::{error::Error, fs::metadata};

mod pair;
use crate::pair::Pair;
mod node;
use crate::node::LNode;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    match args.len() {
        1 => {
            let root_path = std::env::current_dir()?;
            println!("[lightView Manage] {:?}", root_path.as_path());
            build_file_tree(root_path.to_string_lossy().to_string());
        }
        2 => {
            let root_path = args[1].to_owned();
            println!("[lightView Manage] {:?}", root_path);
            build_file_tree(root_path);
        }
        _ => {
            println!("[lightView Failed!]");
            std::process::exit(64);
        }
    }

    Ok(())
}

fn build_file_tree(root_path: String) {
    let metad = metadata(&root_path).expect("[fail] get root path metadata failed");
    if !metad.is_dir() {
        std::process::exit(64);
    }

    // let op = std::process::Command::new("ls").arg(root_path).output();
    //@todo 链接 metad.is_symlink()

    let rds = read_dir(root_path)
        .expect("[fail] read dir failed")
        .collect::<Vec<_>>();
    // let mut node = LNode::new();
    rds.iter().for_each(|i| {
        if let Ok(ir) = i {
            println!("[DEBUG] {:?},{:?}", ir.path(), ir.file_name());
            let mut s = ir.file_name().into_string().unwrap();
            // @todo better check for xxx without '.'
            s.push('.');
            // pass suffix
            let s2 = s.split(".").collect::<Pair<_>>();
            println!("{:?}", s2);
            // node.set_pair(s2);
        }
    });
}

//@todo 自动为文件配置
