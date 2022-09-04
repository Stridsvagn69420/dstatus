use std::fs;
use std::io;
use std::path::Path;
use clap::ArgMatches;
use serde_json;
use discord_rich_presence::activity::Activity;
use crate::meta;

/// Get Values from [ArgMatches]
/// 
/// Reads all values from the command line to an [Activity]
pub fn args(args: ArgMatches) -> io::Result<(Activity<'static>, String)> {
    // Get ID, return Err if not present
    let id = match args.get_one::<String>(meta::ID) {
        Some(x) => x.to_owned(),
        None => {
            return Err(io::Error::new(io::ErrorKind::NotFound, "ID is not present"))
        }
    };

    // Get activity
    let act = activity(
        args.get_one::<String>(meta::DETAILS),
        args.get_one::<String>(meta::STATE),
        args.get_one::<String>(meta::STATE),
        args.get_one::<String>(meta::STATE),
        args.get_one::<String>(meta::STATE),
        args.get_one::<String>(meta::STATE),
        args.get_one::<String>(meta::STATE),
        args.get_one::<String>(meta::STATE),
        args.get_one::<String>(meta::STATE),
        args.get_one::<String>(meta::STATE),
        args.get_one::<i64>(meta::START),
        args.get_one::<i64>(meta::END)
    );

    // Return Activity and ID
    Ok((act, id))
}

/// Get Values from JSON
/// 
/// Reads all values from the given JSON File to an [Activity]
pub fn json(path: impl AsRef<Path>) -> io::Result<(Activity<'static>, String)> {
    // Read file
    let jsonstr = fs::read_to_string(path)?;
    let json: meta::Flags = serde_json::from_str(jsonstr.as_str())?;

    // Get Activity
    let act = activity(
        json.details.as_ref(),
        json.state.as_ref(),
        json.large_image.as_ref(),
        json.large_text.as_ref(),
        json.small_image.as_ref(),
        json.small_text.as_ref(),
        json.btn1_label.as_ref(),
        json.btn1_url.as_ref(),
        json.btn2_label.as_ref(),
        json.btn2_url.as_ref(),
        json.start_time.as_ref(),
        json.end_time.as_ref()
    );
    
    // Return Activity and ID
    Ok((act, json.id))
}

/// Activity
/// 
/// Builds the activity
fn activity(
    details: Option<&String>,
    state: Option<&String>,
    large_image: Option<&String>,
    large_text: Option<&String>,
    small_image: Option<&String>,
    small_text: Option<&String>,
    btn1_label: Option<&String>,
    btn1_url: Option<&String>,
    btn2_label: Option<&String>,
    btn2_url: Option<&String>,
    start_time: Option<&i64>,
    end_time: Option<&i64>
) -> Activity<'static> {
    // Create Activity
    let mut act = Activity::new();

    // TODO: Add a lot of if let checks here

    // Return Activity
    act
}