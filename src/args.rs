use clap::{Arg, ArgAction};

/// File Arg
/// 
/// Argument to use a JSON file instead to provide the Rich Presence data.
pub fn file() -> Arg<'static> {
    Arg::new("file")
    .long("file")
    .takes_value(true)
    .value_name("FILE")
    .help("Uses given JSON file that provides the Rich Presence data")
    .action(ArgAction::Set)
}

/// Details Arg
/// 
/// Provides the Rich Presence Details.
pub fn details() -> Arg<'static> {
    Arg::new("details")
    .long("details")
    .takes_value(true)
    .value_name("DETAILS")
    .help("Provides the Rich Presence details")
}

/// State Arg
/// 
/// Provides the Rich Presence State.
pub fn state() -> Arg<'static> {
    Arg::new("state")
    .long("state")
    .takes_value(true)
    .value_name("STATE")
    .help("Provides the Rich Presence state")
}

/// Large Image Arg
/// 
/// Sets the large image.
pub fn large_image() -> Arg<'static> {
    Arg::new("large_image")
    .long("large_image")
    .takes_value(true)
    .value_name("IMAGE")
    .help("Sets the large image")
}

/// Large Image Text Arg
/// 
/// Sets the large image.
pub fn large_text() -> Arg<'static> {
    Arg::new("large_text")
    .long("large_text")
    .takes_value(true)
    .value_name("IMAGE")
    .help("Sets the large image's text")
}

/// Small Image Arg
/// 
/// Sets the large image.
pub fn small_image() -> Arg<'static> {
    Arg::new("small_image")
    .long("small_image")
    .takes_value(true)
    .value_name("IMAGE")
    .help("Sets the small image")
}

/// Small Image Text Arg
/// 
/// Sets the large image.
pub fn small_text() -> Arg<'static> {
    Arg::new("small_text")
    .long("small_text")
    .takes_value(true)
    .value_name("IMAGE")
    .help("Sets the small image's text")
}