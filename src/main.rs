use std::collections::HashMap;

struct Cli {
    root_path: std::path::PathBuf,
}

#[allow(dead_code, unused_mut)]
fn find_all_images() -> HashMap<String, u8>{
    let mut m = HashMap::new();
    m
}

fn main() {
    let path = std::env::args().nth(1).expect("Specify root folder");
    let args = Cli {
        root_path: std::path::PathBuf::from(path),
    };
    println!("root : {:?}", args.root_path);
}
