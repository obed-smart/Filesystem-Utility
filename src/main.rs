#[warn(unused_variables)]
#[warn(unused_imports)]
pub mod cli;
mod create;
pub mod functions;
mod write;

use create::handle_create;
use write::handle_write;

use cli::build_cli_command;

fn main() {
    let matche_command = build_cli_command().get_matches();

    match matche_command.subcommand() {
        Some(("create", sub_match)) => {
            handle_create(sub_match);
        }
        Some(("write", sub_match)) => {
            handle_write(sub_match);
        }

        _ => eprintln!("❌ Unknown command. Use --help to see available options."),
    }
}
