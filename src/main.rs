use std::{collections::HashMap, ffi::OsStr, fs::{self, read_to_string}, path:: PathBuf};

use clap::{arg, command, value_parser};

struct Cli {
    root_path: std::path::PathBuf,
}

fn get_ignores(root_path: std::path::PathBuf) -> HashMap<String,u8>{
    let mut ignores: HashMap<String, u8> = HashMap::from([
        (".git".to_string(),0)
    ]);
    let gitignore = root_path.join(".gitignore");

    if gitignore.exists() {
        for line in read_to_string(&gitignore).unwrap().lines(){
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() && !trimmed_line.starts_with('#'){
                ignores.insert(trimmed_line.to_string(), 0);
            }
        }
    } else {
        println!("Did not found .gitignore")
    }
    ignores
}

fn find_all_images(root_path: std::path::PathBuf) -> HashMap<String, u8>{
    let images_types: HashMap<&str,bool> = HashMap::from([
        ("jpg", true),
        ("jpeg",true),
        ("png", true),
        ("svg", true),
    ]);
    let mut images: HashMap<String, u8> = HashMap::new();
    let mut queue: Vec<PathBuf> = Vec::new();
    queue.push(root_path);
    while let Some(dir) = queue.pop(){
        if let Ok(entries) = fs::read_dir(&dir){
            for entry in entries.flatten(){
                let path = entry.path();
                if path.is_dir(){
                    queue.push(path);
                    continue;
                }

                let Some(ext_str) = path
                    .extension()
                    .and_then(OsStr::to_str) else {
                        continue;
                };

                if images_types.contains_key(ext_str)
                    && let Some(path_str) = path.to_str(){
                        images.insert(path_str.to_string(), 1);
                    }
                

            }
        }
    }
    images
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
    let i = get_ignores(args.root_path.clone());
    i.iter().for_each(|(k,_)|{
        println!("{:?}", k);
    });

    find_all_images(args.root_path.clone());
}
