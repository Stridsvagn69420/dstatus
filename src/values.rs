use std::io;
use std::path::Path;
use clap::ArgMatches;
use discord_rich_presence::activity::Activity;
use crate::meta;

/// Get Values from [ArgMatches]
/// 
/// Reads all values from the command line to an [Activity]
pub fn args(args: ArgMatches) -> io::Result<(Activity<'static>, String)> {
    // TODO: Add logic here
}

/// Get Values from JSON
/// 
/// Reads all values from the given JSON File to an [Activity]
pub fn json(path: impl AsRef<Path>) -> io::Result<(Activity<'static>, String)> {
    // TODO: Add logic here
}