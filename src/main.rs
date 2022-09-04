use std::sync::mpsc::channel;
use std::process::ExitCode;
use ctrlc;
use clap::{Command, crate_authors, crate_description, crate_name, crate_version, ColorChoice};
use discord_rich_presence::{DiscordIpcClient, DiscordIpc};
use kagero::printer::{Printer, Colors};

mod meta;
mod values;

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

    // Create RPC Activity and get App ID
    let rpcdata = match matches.get_one::<String>(meta::FILE) {
        Some(path) => values::json(path),
        None => values::args(matches)
    };

    // Check for errors
    let unwraped_rpcdata = match rpcdata {
        Ok(x) => x,
        Err(_) => {
            // TODO: Print error message
            return ExitCode::FAILURE;
        }
    };

    // Create RPC Client
    let mut rpc = match DiscordIpcClient::new(&unwraped_rpcdata.1) {
        Ok(client) => client,
        Err(_) => {
            printer.errorln("Provided Application ID is not valid!", Colors::RedBright);
            return ExitCode::FAILURE;
        }
    };

    // Set RPC Activity
    match rpc.set_activity(unwraped_rpcdata.0) {
        Ok(()) => {
            printer.println("Successfully set the activity!", Colors::Green);
            printer.println("You can quit the application with Ctrl+C.", Colors::Cyan);
        },
        Err(_) => {
            printer.errorln("Could not set the activity...", Colors::RedBright);
            // TODO: Print out error
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
