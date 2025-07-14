use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

// walk throug from the root directory to find the folder
// return if not None found
fn find_folder_by_name(path: String) -> io::Result<Option<PathBuf>> {
    for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            if let Some(name) = entry_path.file_name() {
                if name.to_string_lossy() == path {
                    return Ok(Some(entry_path.to_path_buf()));
                }
            }
        }
    }

    Ok(None)
}
fn find_in_folder(base: String, path: String) -> io::Result<Option<PathBuf>> {
    let base_path = Path::new(&base);

    if !base_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{:#?} does not exist", base_path),
        ));
    }

    if !base_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory,
            format!("{:#?} is not a valid directory", base_path),
        ));
    }

    for entry in WalkDir::new(base).into_iter().filter_map(Result::ok) {
        let entry_path = entry.path();

        println!("entry: {:#?}", entry_path.file_name());

        if let Some(name) = entry_path.file_name() {
            let name_str = name.to_string_lossy();
            if name_str == path {
                return Ok(Some(entry_path.to_path_buf()));
            }
        }
    }

    Ok(None)
}

// create a folder on the root
pub fn create_folder(path: &String) -> io::Result<()> {
    let folder_path = Path::new(path);

    if folder_path.exists() {
        println!("❌ This folder already exist here");
    } else {
        fs::create_dir_all(folder_path)?;
    }
    Ok(())
}

// create a file on the root
pub fn handle_create_file(path: String) -> io::Result<()> {
    let file_path = Path::new(&path);

    if file_path.exists() {
        println!("❌ This file already exist here");
    } else {
        File::create(file_path)?;
    }
    Ok(())
}

// create a folder into the folder selected
// if its exists return err
// if its not found return err
pub fn handle_create_in_to_folder(in_to_folder: String, folder: String) -> io::Result<()> {
    match find_folder_by_name(in_to_folder) {
        Ok(Some(path)) => {
            let folder_path = path.join(folder);

            if folder_path.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "This folder already exist here",
                ));
            } else {
                fs::create_dir(folder_path)?;
            }
        }
        Ok(None) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "❌ Folder not found",
            ));
        }
        Err(e) => eprintln!("Error {}", e),
    };

    Ok(())
}

// create a file into the folder selected
// if its exists return err
// if its not found return err
pub fn handle_create_in_to_file(in_to_folder: String, file: String) -> io::Result<()> {
    match find_folder_by_name(in_to_folder) {
        Ok(Some(path)) => {
            let file_path = path.join(file);

            if file_path.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "This file already exist here",
                ));
            } else {
                File::create(file_path)?;
            }
        }
        Ok(None) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "❌ Folder not found",
            ));
        }
        Err(e) => eprintln!("Error {}", e),
    };

    Ok(())
}

pub fn handle_write_file(file: String, content: &Vec<String>) -> io::Result<()> {
    let file_path = Path::new(&file);

    if file_path.exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)?;

        let combined = content.join(" ") + "\n";

        file.write_all(combined.as_bytes())?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "❌ {} does not exist on the root please use --in (folder) to search by folder or --in (.) to search from the root",
                file_path.display()
            ),
        ));
    }

    Ok(())
}

pub fn handle_write_in_to_file(
    file: String,
    in_to: String,
    content: &Vec<String>,
) -> io::Result<()> {
    match find_in_folder(in_to, file) {
        Ok(path) => {
            println!("{:#?}", path);
            if let Some(file_path) = path {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(file_path)?;

                let combined = content.join(" ") + "\n";

                file.write_all(combined.as_bytes())?;
            } else {
                eprintln!("❌ {:#?} Path was not found", path);
            }
            Ok(())
        }

        Err(e) => {
            eprintln!("❌ An error occured: {}", e);
            Err(e)
        }
    }
}
