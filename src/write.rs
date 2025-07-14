use clap::ArgMatches;

use crate::functions::*;

pub fn handle_write(sub_match: &ArgMatches) {
    let contents: Vec<_> = sub_match
        .get_many::<String>("content")
        .unwrap_or_default()
        .map(|c| c.to_string())
        .collect();
    let file_name = sub_match.get_one::<String>("file");
    let in_to_folder = sub_match.get_one::<String>("in");

    if in_to_folder.is_none() {
        if let Some(file) = file_name {
            if file.trim().is_empty() {
                println!("❌ file name can not be empty")
            }
            if !file.contains('.') {
                eprintln!(
                    "❌ File name '{}' must include an extension like  a (.)extension name",
                    file
                );
                return;
            }

            if !contents.is_empty() {
                match handle_write_file(file.to_string(), &contents) {
                    Ok(_) => {
                        println!("✅ New content added to {}", file)
                    }

                    Err(e) => eprint!("❌ {}", e),
                }
            }
        }
    }

    if let (Some(folder), Some(file)) = (in_to_folder, file_name) {
        if folder.trim().is_empty() {
            println!("❌ Folder name can not be empty");
            return;
        }

        if folder.contains('.') {
            eprintln!("❌ Folder name '{}' must not contain a dot (.)", folder);
            return;
        }

        if !file.contains('.') {
            eprintln!(
                "❌ File name '{}' must include an extension like  a (.)extension name",
                file
            );
            return;
        }

        if !contents.join(" ").trim().is_empty() {
            match handle_write_in_to_file(file.to_string(), folder.to_string(), &contents) {
                Ok(_) => {
                    println!("✅ New content added to {}", file)
                }

                Err(e) => eprint!("❌ {}", e),
            }
        }
    }
}
