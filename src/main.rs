use std::sync::mpsc::channel;
use std::process::ExitCode;
use ctrlc;

mod args;

fn main() -> ExitCode {
    // Set Channel for Ctrl-C
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).unwrap()).unwrap();

    rx.recv().unwrap();

    ExitCode::SUCCESS
}
