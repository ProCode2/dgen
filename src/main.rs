use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct FileNode {
    name: String,
    content: String,
}

// The core data structure
#[derive(Serialize, Deserialize, Debug)]
struct FolderNode {
    name: String,
    files: Vec<FileNode>,
    folders: Vec<FolderNode>,
}

// the core algorithm
fn dir_from_json(folder: &FolderNode, path: String) -> std::io::Result<()> {
    let new_path = format!("{}{}/", path, &folder.name);

    // create the new directory
    println!("Generating new directory: {}", &new_path);
    fs::create_dir(&new_path)?;

    // create all the files in this directory
    folder.files.iter().for_each(|file| {
        println!("Generating new file: {}{}", &new_path, &file.name);
        let mut file_handler = fs::File::create(format!("{}{}", &new_path, &file.name)).unwrap();
        let _ = file_handler.write_all(&file.content.as_bytes()).unwrap();
    });

    // base case [TODO: also handle if folder key does not exist]
    if folder.folders.len() == 0 {
        return Ok(());
    }

    // recursively calling gen_template
    folder.folders.iter().for_each(|node| {
        let _ = dir_from_json(&node, new_path.to_string());
    });

    Ok(())
}

fn json_from_dir(folder: &mut FolderNode, path: String) {
    let new_path = format!("{}{}/", path, folder.name);
    println!("{}", &new_path);
    let paths = fs::read_dir(&new_path).expect(&new_path);

    for res_path in paths {
        let path = res_path.unwrap();
        if fs::metadata(path.path()).unwrap().is_dir() {
            let mut f = FolderNode {
                name: path
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                files: Vec::new(),
                folders: Vec::new(),
            };
            json_from_dir(&mut f, new_path.clone());
            folder.folders.push(f);
        } else {
            if fs::read_to_string(path.path()).is_err() {
                continue;
            }
            let file_content = fs::read_to_string(path.path()).unwrap();
            folder.files.push(FileNode {
                name: path
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                content: file_content,
            })
        }
    }
}

fn generate_json() {
    let root_name = env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let mut f = FolderNode {
        name: String::from(&root_name),
        files: Vec::new(),
        folders: Vec::new(),
    };

    json_from_dir(&mut f, String::from("../"));
    let json_template = serde_json::to_string(&f).unwrap();
    match fs::write(format!("{}.json", root_name), &json_template) {
        Ok(_) => println!("Done!"),
        Err(_) => {
            println!("Failed to write json into file");
            println!("Printing the result to console instead:");
            println!("{}", &json_template);
        }
    }
}

fn generate_dir(json_path: &str) {
    let json_data = fs::read_to_string(&json_path).expect("smth wrong with that file, hmm sus...");
    let handler: FolderNode = serde_json::from_str(&json_data).unwrap();
    let init_path = format!("{}/", env::current_dir().unwrap().display().to_string());
    let _ = dir_from_json(&handler, init_path);
}

fn main() -> Result<()> {
    let matches = App::new("Dgen")
        .version("1.0")
        .author("ProCode")
        .about("Create your starter repositories from a single json blueprint.")
        .arg(
            Arg::with_name("Generate_JSON")
                .short("b")
                .long("blueprint")
                .help("Create json blueprint of the directory you are in."),
        )
        .arg(
            Arg::with_name("Generate_Repository")
                .short("g")
                .long("generate")
                .value_name("path to JSON blueprint")
                .help("Create the directory from the json blueprint.")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("Generate_JSON") {
        generate_json();
    } else if matches.is_present("Generate_Repository") {
        let json_path = matches
            .value_of("Generate_Repository")
            .unwrap_or("template.json");
        generate_dir(json_path);
    }

    Ok(())
}
