use std::sync::mpsc::channel;
use std::process::ExitCode;
use ctrlc;
use clap::{Command, crate_authors, crate_description, crate_name, crate_version, ColorChoice};
use discord_rich_presence::{DiscordIpcClient, DiscordIpc};
use kagero::printer::{Printer, Colors};

mod meta;
mod values;
mod data;

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
        meta::id(),
        meta::file(),
        meta::details(),
        meta::state(),
        meta::large_image(),
        meta::large_text(),
        meta::small_image(),
        meta::small_text(),
        meta::btn1_label(),
        meta::btn1_url(),
        meta::btn2_label(),
        meta::btn2_url(),
        meta::start(),
        meta::end()
    ]);

    // Set Channel for Ctrl-C
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).unwrap()).unwrap();

    // Parse command-line arguments
    let matches = clapcmd.get_matches();
    let mut printer = Printer::default();
    let mut rpcdata = data::Data::empty();
    if let Some(filepath) = matches.get_one::<String>(meta::FILE) {
        // TODO: Read provided JSON file, return ExitCode::FAILURE if reading or parsing fails
    } else {
        if let Err(_) = values::args(&mut rpcdata, matches) {
            printer.errorln("No App ID was provided!", Colors::RedBright);
            return ExitCode::FAILURE;
        }
    }

    // Set status
    let mut rpc = match DiscordIpcClient::new(&rpcdata.id) {
        Ok(client) => client,
        Err(_) => {
            printer.errorln("Provided Application ID is not valid!", Colors::RedBright);
            return ExitCode::FAILURE;
        }
    };
    let activity = rpcdata.to_activity();
    match rpc.set_activity(activity) {
        Ok(()) => {
            printer.println("Successfully set the activity!", Colors::Green);
            printer.println("You can quit the application with Ctrl+C.", Colors::Cyan);
        },
        Err(_) => {
            printer.errorln("Could not set the activity...", Colors::RedBright);
            return ExitCode::FAILURE;
        }
    }

    // Exit on SIGINT, SIGTERM and SIGHUP
    rx.recv().unwrap();
    if let Ok(_) = rpc.close() {
        printer.println("Successfully logged out of Discord!", Colors::CyanBright);
    }
    ExitCode::SUCCESS
}
