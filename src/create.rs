use clap::ArgMatches;

use crate::functions::*;

pub fn handle_create(sub_match: &ArgMatches) {
    let folder_name = sub_match.get_one::<String>("folder");
    let folders_name: Vec<_> = sub_match
        .get_many::<String>("folders")
        .unwrap_or_default()
        .map(|f| f.to_string())
        .collect();
    let file_name = sub_match.get_one::<String>("file");
    let in_to_folder = sub_match.get_one::<String>("in");

    if !folders_name.is_empty() {
        if folders_name.len() > 1 {
            for folder in folders_name {
                println!("✅ creating folder:{:#?}", folder);
                // match create_folder(&folder.to_string()) {
                //     Ok(_) => println!("✅ creating folder:{:#?}", folder),
                //     Err(e) => eprintln!("{e}"),
                // }
            }
        } else {
            eprintln!(" ❌ You can not create one folder with this command");
            return;
        }
    }

    if let Some(parent_folder) = in_to_folder {
        println!("📁 New folder created: parent {} ", parent_folder)
    }

    if let Some(folder) = folder_name {
        if folder.trim().is_empty() {
            println!("❌ Folder name can not be empty");
            return;
        }

        if folder.contains('.') {
            eprintln!("❌ Folder name '{}' must not contain a dot (.)", folder);
            return;
        }
    }

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

        println!("📁 New file created: {}", file)
    }

    if in_to_folder.is_some() && folder_name.is_none() && file_name.is_none() {
        eprintln!("❌ You must provide either --folder or --file when using --in");
        return;
    } else if let (Some(folder), Some(in_to)) = (folder_name, in_to_folder) {
        if let Some(file) = file_name {
            match handle_create_in_to_file(in_to.to_string(), file.to_string()) {
                Ok(_) => println!(
                    "✅ New file {} and folder {} was created and inserted in {} folder",
                    file, folder, in_to
                ),
                Err(e) => eprintln!("❌ Error: {}", e),
            }
        }

        match handle_create_in_to_folder(in_to.to_string(), folder.to_string()) {
            Ok(_) => println!(
                "✅ New folder {:?} was created and inserted in {:?} folder",
                folder_name, in_to
            ),
            Err(e) => eprintln!("❌ Error : {e}"),
        }
    } else if let (Some(file), Some(in_to)) = (file_name, in_to_folder) {
        match handle_create_in_to_file(in_to.to_string(), file.to_string()) {
            Ok(_) => println!(
                "✅ New file {} was created and inserted in {} folder",
                file, in_to
            ),
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    } else if let (Some(file), Some(folder)) = (file_name, folder_name) {
        println!(
            "✅ New file {:?} was created and inserted in {:?} folder",
            file, folder
        )
    } else if let Some(folder) = folder_name {
        match create_folder(folder) {
            Ok(_) => println!("✅ New folder created successfullyF"),
            Err(e) => eprintln!("{e}"),
        }
    } else if let Some(file) = file_name {
        match handle_create_file(file.to_string()) {
            Ok(_) => println!("✅ New file created successfully"),
            Err(e) => eprint!(" Failed when creating file : {}", e),
        }
    }
}
