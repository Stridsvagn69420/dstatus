use clap::ArgMatches;
use crate::data::Data;
use crate::meta;

/// Get Values from [ArgMatches]
/// 
/// Reads all values from the command line as [Data]
pub fn args(data: &mut Data, args: ArgMatches) -> Result<(), ()> {
    // Set ID, return Err if not present
    if let Some(id) = args.get_one::<String>(meta::ID) {
        data.id = id.to_string();
    } else {
        return Err(());
    }

    // Sets the State
    if let Some(x) = args.get_one::<String>(meta::STATE) {
        data.state = Some(x.to_string())
    }

    // Return Ok
    Ok(())
}