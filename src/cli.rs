use clap::{Arg, ArgAction, Command};

pub fn build_cli_command() -> Command {
    Command::new("filesystem-utility")
        .about("A Rust-based CLI Scaffolding and File System Utility Tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("Use to creat a folder or file")
                .arg(
                    Arg::new("folder")
                        .long("folder")
                        .help("Name of the folder to create")
                        .value_name("FOLDER")
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("folders")
                        .long("folders")
                        .help("List of folder names")
                        .value_name("FOLDERS")
                        .num_args(1..)
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("file")
                        .long("file")
                        .help("Name of file")
                        .value_name("FILE")
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("in")
                        .long("in")
                        .help("Parent folder to create the new folder or file in")
                        .required(false)
                        .value_name("In")
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("write")
                .about("Used to write to a file")
                .arg(
                    Arg::new("file")
                        .long("file")
                        .help("Name of file to write to ")
                        .value_name("FILE")
                        .required(true)
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("in")
                        .long("in")
                        .help("Parent folder to write the content to")
                        .required(false)
                        .value_name("In")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("content")
                        .long("content")
                        .help("the content to write in the file")
                        .required(true)
                        .value_name("CONTENT")
                        .action(ArgAction::Append),
                ),
        )
}
