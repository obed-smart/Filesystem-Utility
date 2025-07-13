use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

// walk throug from the root directory to find the folder
// return if not None found
fn find_folder_by_name(path: String) -> io::Result<Option<PathBuf>> {
    for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
        let dir_path = entry.path();
        if dir_path.is_dir() {
            if let Some(name) = dir_path.file_name() {
                if name.to_string_lossy() == path.as_str() {
                    return Ok(Some(dir_path.to_path_buf()));
                }
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
                    "This ffolder already exist here",
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
