use std::{collections::HashMap, ffi::{OsStr, OsString}, fs::{self, read_to_string}, path:: PathBuf, sync::{LazyLock, OnceLock}};

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
                let normalized = trimmed_line.strip_suffix('/').unwrap_or(trimmed_line);
                ignores.insert(normalized.to_string(), 0);
            }
        }
    } else {
        eprintln!("Did not found .gitignore")
    }
    ignores
}

static IGNORES: OnceLock<HashMap<String, u8>> = OnceLock::new();

fn find_all_images(root_path: std::path::PathBuf) -> HashMap<String, PathBuf>{
    let images_types: HashMap<&str,bool> = HashMap::from([
        ("jpg", true),
        ("jpeg",true),
        ("png", true),
        ("svg", true),
    ]);
    let mut images = HashMap::new();
    let is_image = |path: PathBuf| {
            let Some(extension_str) = path
                .extension()
                .and_then(OsStr::to_str) else {
                    return;
            };

            if images_types.contains_key(extension_str)
                && let Some(file_name) = path.file_name(){
                    images.insert(file_name.to_string_lossy().to_string(), path.clone());
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

fn find_images_in_files(root_path: PathBuf) -> HashMap<PathBuf, Vec<String>> {
    let mut imgs_to_files = HashMap::new();
    let extract_images = |path: PathBuf| {
        let imgs = find_images_in_file(&path);    
        if !imgs.is_empty(){
            imgs_to_files.insert(path.clone(), imgs);
        }
    };
    abstract_bfs(root_path, extract_images);
    imgs_to_files
}


fn find_images_in_file(file_path: &PathBuf) -> Vec<String>{
    let mut res = Vec::new();
    if file_path.extension().and_then(|ext| ext.to_str()) != Some("md") {
        return res;
    }
    let Ok(content) = read_to_string(&file_path) else {
        eprintln!("Couldn't read file {:?}", file_path);
        return res;
    };
    let re = Regex::new(r"!\[\[(?<path>.*?)\]\]").unwrap();
    for caps in re.captures_iter(&content){
         res.push(caps["path"].to_string()); 
    }
    res
}

fn move_file(src: PathBuf, mut dest: PathBuf){
    dest.pop();
    if let Some(file_name) = src.file_name(){
        dest.push(file_name);
        if src != dest{
            if let Ok(_) = std::fs::rename(&src, &dest) {
                println!("Moved: {} -> {}", src.display(), dest.display());
            } else {
                eprintln!("Couldn't move {} to {} as expected", src.display(), dest.display());
            }
        }
    }
}

fn move_files(
    notes:  HashMap<PathBuf, Vec<String>>,
    images: HashMap<String, PathBuf>
){
    for (src, imgs) in notes.into_iter(){
        for img in imgs.iter() {
            if let Some(img_path) = images.get(img){
                move_file(img_path.clone(), src.clone());
            }
        } 
    }
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
    let Ok(_) = IGNORES.set(get_ignores(args.root_path.clone())) else {
        eprintln!("Something wrong happened..");
        return;
    };

    let images = find_all_images(args.root_path.clone());

    let notes = find_images_in_files(args.root_path.clone());

    move_files(notes, images);
}
