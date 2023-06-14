use std::result::Result;
use std::error::Error;

pub mod node;
pub mod trie;
pub mod notifyer;
pub mod builder;
pub mod tom;
pub use crate::node::Node;
pub use crate::trie::Trie;
pub use crate::builder::Builder;
pub use crate::notifyer::Notifyer;

fn main() -> Result<(), Box<dyn Error>> {

    let args = std::env::args().collect::<Vec<_>>();

    match args.len() {
        1 => {
            let root_path = std::env::current_dir()?;
            println!("[lightView Manage] {:?}", root_path.as_path());

            Builder::build_with_dir(root_path.to_str().unwrap())
                .on_config("");

            let thd = std::thread::spawn(move ||{
                println!("[lightView Watch] {:?}", root_path.as_path());
                Notifyer::new().watch(root_path.to_str().unwrap());
            });
            
            println!("[lightView Runing...]");
            thd.join();
        }
        2 => {
            let root_path = args[1].to_owned();
            println!("[lightView Manage] {:?}", root_path);
            Builder::build_with_dir(&root_path);
        }
        _ => {
            println!("[lightView Failed!]");
            std::process::exit(64);
        }
    }

    Ok(())
}


/*
fn build_file_tree_custom(root_path: String) {
    let metad = metadata(&root_path).expect("[fail] get root path metadata failed");
    if !metad.is_dir() {
        std::process::exit(64);
    }

    // let op = std::process::Command::new("ls").arg(root_path).output();
    //@todo 链接 metad.is_symlink()

    let rds = read_dir(root_path)
        .expect("[fail] read dir failed")
        .collect::<Vec<_>>();
    let mut node = Node::new();
    rds.iter().for_each(|i| {
        if let Ok(ir) = i {
            println!("[DEBUG] {:?},{:?}", ir.path(), ir.file_name());
            // node.file_own = ir.file_name().into_string().unwrap();
            // @todo better check for xxx without '.'
            // s.push('.'); // pass suffix
            // let s2 = s.split(".").to_owned().collect::<Pair<_>>();
        }
    });
    // node.set_pair(node.file_own.split('.').collect::<Pair<_>>());
}
 */
//@todo 自动为文件配置
//@todo 序列化保存到.view
