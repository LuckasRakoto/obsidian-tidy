use std::{collections::HashMap, fs, path:: PathBuf};

use clap::{arg, command, value_parser};

struct Cli {
    root_path: std::path::PathBuf,
}

fn find_all_images(root_path: std::path::PathBuf) -> HashMap<String, u8>{
    let m = HashMap::new();
    let mut queue: Vec<PathBuf> = Vec::new();
    queue.push(root_path);
    while let Some(dir) = queue.pop(){
        let read_result = fs::read_dir(&dir);
        if let Ok(entries) = read_result {
            for entry in entries.flatten() {
                println!("{}",entry.path().display());
            }
        } else {
            eprintln!("Failed to read_dir {}", dir.display());
        }
    }
    m
}


fn main() {
    let matches = command!()
        .arg(
            arg!(
                -s --source <File> "Sets the base path"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf))
        )
        .get_matches();
    let mut args = Cli { root_path : PathBuf::from(".")};
    if let Some(source) = matches.get_one::<PathBuf>("source"){
        args.root_path = source.to_path_buf();
    }
    println!("Source value : {}", args.root_path.display());
    find_all_images(args.root_path);
}
