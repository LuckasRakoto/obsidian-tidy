#![allow(dead_code, unused)]

use std::{collections::HashMap, ffi::OsStr, fs::{self, read_to_string}, path:: PathBuf, sync::{LazyLock, OnceLock}};

use clap::{arg, command, value_parser};
use regex::Regex;

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

static IGNORES: OnceLock<HashMap<String, u8>> = OnceLock::new();

fn find_all_images(root_path: std::path::PathBuf) -> HashMap<String, u8>{
    let images_types: HashMap<&str,bool> = HashMap::from([
        ("jpg", true),
        ("jpeg",true),
        ("png", true),
        ("svg", true),
    ]);
    let mut images: HashMap<String, u8> = HashMap::new();
    let is_image = |path: PathBuf| {
            let Some(extension_str) = path
                .extension()
                .and_then(OsStr::to_str) else {
                    return;
            };

            if images_types.contains_key(extension_str)
                && let Some(path_str) = path.to_str(){
                    images.insert(path_str.to_string(), 1);
                }
    };
    abstract_bfs(root_path, is_image);
    images
}

fn abstract_bfs<F>(root_path: std::path::PathBuf, mut func: F)
where F: FnMut(PathBuf),
{
    let mut queue: Vec<PathBuf> = Vec::new();
    queue.push(root_path);
    while let Some(dir) = queue.pop(){
        if let Ok(entries) = fs::read_dir(&dir){
            for entry in entries.flatten(){
                let path = entry.path();
                if let (Some(ignore), Some(name)) = (IGNORES.get(), path.file_name()) &&
                    ignore.contains_key(name.to_string_lossy().as_ref()) {
                        println!("Ignoring : {}", path.display());
                    continue;
                }
                if path.is_dir(){
                    queue.push(path);
                    continue;
                }

                func(path);
                
            }
        }
    }
}

fn find_images_in_files(root_path: PathBuf) -> HashMap<PathBuf, PathBuf> {
    let mut files_to_img = HashMap::new();

    files_to_img
}


fn find_images_in_file(file_path: &str) -> Vec<String>{
    let mut res = Vec::new();
    let Ok(content) = read_to_string(file_path) else {
        eprintln!("Couldn't read file {:?}", file_path);
        return res;
    };
    let re = Regex::new(r"!\[\[(?<path>.*?)\]\]").unwrap();
    for caps in re.captures_iter(&content){
         res.push(caps["path"].to_string()); 
    }
    res
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
    IGNORES.set(get_ignores(args.root_path.clone()));

    let images = find_all_images(args.root_path.clone());
    images.iter().for_each(|(k,_)|{
        println!("{:?}", k);
    });


}
