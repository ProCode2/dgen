use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{prelude::*, Error as IOError, ErrorKind as IOErrorKind};
use std::path::Path;
use std::process::Command;
use tempdir::TempDir;

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
fn dir_from_json(folder: &FolderNode, path: String) -> Result<(), IOError> {
    let new_path = format!("{}{}/", path, &folder.name);

    // create the new directory
    println!("Generating new directory: {}", &new_path);
    fs::create_dir(&new_path)?;

    // create all the files in this directory
    folder.files.iter().for_each(|file| {
        println!("Generating new file: {}{}", &new_path, &file.name);
        let mut file_handler = fs::File::create(format!("{}{}", &new_path, &file.name)).unwrap();
        let _ = file_handler.write_all(file.content.as_bytes()).unwrap();
    });

    // base case [TODO: also handle if folder key does not exist]
    if folder.folders.is_empty() {
        return Ok(());
    }

    // recursively calling gen_template
    folder.folders.iter().for_each(|node| {
        let _ = dir_from_json(node, new_path.to_string());
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

fn generate_json(path: &Path) {
    let root_name = path.file_name().unwrap().to_string_lossy();
    let mut f = FolderNode {
        name: String::from(root_name.clone()),
        files: Vec::new(),
        folders: Vec::new(),
    };

    json_from_dir(
        &mut f,
        format!("{}/", path.parent().unwrap().to_string_lossy()),
    );
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

/// Determines if the passed string is in the form suitable
/// for fetching from GitHub.
fn valid_repository_reference(reference: &str) -> Option<String> {
    if reference.matches('/').count() != 1 {
        return None;
    }
    let mut split = reference.split('/');
    let mut next = None;
    for _ in 0..2 {
        next = split.next();
        if next.is_none() || next.unwrap().is_empty() {
            return None;
        }
    }
    next.map(str::to_owned)
}

/// Generates JSON from a repository by first cloning it
/// to a temporary directory on the user's system.
fn generate_from_repository(reference: &str, repo_name: &str) -> Result<(), IOError> {
    let temp = TempDir::new("dgen")?;
    let temp_path = format!("{}", temp.path().display());
    let repo = format!("https://github.com/{}", reference);
    let return_code = Command::new("git")
        .args(["clone", &repo, &format!("{}/{}", &temp_path, repo_name)].iter())
        .status()?;
    if !return_code.success() {
        return Err(IOError::new(
            IOErrorKind::Other,
            format!("Git clone command failed with exit code {}", return_code),
        ));
    }
    generate_json(&temp.path().join(repo_name));
    let output_file_name = format!("{}.json", repo_name);
    fs::rename(&temp.path().join(&output_file_name), &output_file_name)
}

fn main() {
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
            Arg::with_name("Generate_JSON_from_repository")
                .short("r")
                .long("repository")
                .value_name("username/repo of GitHub repository")
                .help("Create json blueprint of a GitHub repository.")
                .takes_value(true),
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
        generate_json(&env::current_dir().unwrap());
    } else if matches.is_present("Generate_JSON_from_repository") {
        let reference = matches.value_of("Generate_JSON_from_repository").unwrap();
        let repo_name = match valid_repository_reference(reference) {
            Some(r) => r,
            None => {
                println!(
                    "Invalid repository reference, expected a string in the form 'username/repo'"
                );
                return;
            }
        };
        if let Err(e) = generate_from_repository(reference, &repo_name) {
            println!("Processing the repository failed: {}", e);
        }
    } else if matches.is_present("Generate_Repository") {
        let json_path = matches
            .value_of("Generate_Repository")
            .unwrap_or("template.json");
        generate_dir(json_path);
    }
}

#[cfg(test)]
mod tests {
    use super::valid_repository_reference;

    #[test]
    fn test_valid_repository_reference() {
        assert!(valid_repository_reference("a/b").is_some());
        assert!(valid_repository_reference("a/").is_none());
        assert!(valid_repository_reference("/b").is_none());
        assert!(valid_repository_reference("a/b/c").is_none());
        assert!(valid_repository_reference("ab").is_none());
    }
}
