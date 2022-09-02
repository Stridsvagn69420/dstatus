use std::sync::mpsc::channel;
use std::process::ExitCode;
use ctrlc;
use clap::{Command, crate_authors, crate_description, crate_name, crate_version, ColorChoice};
use discord_rich_presence::{DiscordIpcClient, DiscordIpc};
use kagero::printer::{Printer, Colors};

mod args;
mod json;
mod tasks;

fn main() -> ExitCode {
    // Create clap-rs command
    let clapcmd = Command::new(crate_name!())
    .bin_name(crate_name!())
    .display_name(crate_name!())
    .version(crate_version!())
    .about(crate_description!())
    .author(crate_authors!("\n"))
    .color(ColorChoice::Always)
    .disable_colored_help(false)
    .arg_required_else_help(true)
    .disable_help_subcommand(false)
    .disable_help_flag(false)
    .disable_version_flag(false)
    // Add flags
    .args(vec![
        args::id(),
        args::file(),
        args::details(),
        args::state(),
        args::large_image(),
        args::large_text(),
        args::small_image(),
        args::small_text(),
        args::btn1_label(),
        args::btn1_url(),
        args::btn2_label(),
        args::btn2_url(),
        args::start(),
        args::end()
    ]);

    // Set Channel for Ctrl-C
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).unwrap()).unwrap();

    // Parse command-line arguments
    let matches = clapcmd.get_matches();
    let data = tasks::Data::empty();
    let mut printer = Printer::default();
    if let Some(filepath) = matches.get_one::<String>(args::FILE) {
        // TODO: Read provided JSON file, return ExitCode::FAILURE if reading or parsing fails
    } else {
        // TODO: Read flags, return ExitCode::FAILURE if no App ID was set
    }

    // Set status
    let mut rpc = match DiscordIpcClient::new(&data.id) {
        Ok(client) => client,
        Err(_) => {
            printer.errorln("Provided Application ID is not valid!", Colors::RedBright);
            return ExitCode::FAILURE;
        }
    };

    // Exit on SIGINT, SIGTERM and SIGHUP
    rx.recv().unwrap();
    if let Ok(_) = rpc.close() {
        printer.println("Successfully logged out of Discord!", Colors::CyanBright);
    }
    ExitCode::SUCCESS
}
