use std::{collections::HashMap, path::PathBuf};

use clap::{arg, command, value_parser};

#[allow(unused)]
struct Cli {
    root_path: std::path::PathBuf,
}

#[allow(dead_code, unused_mut)]
fn find_all_images() -> HashMap<String, u8>{
    let mut m = HashMap::new();
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
}
