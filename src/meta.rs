use serde::Deserialize;
use clap::{Arg, ArgAction};

/// Flags JSON Struct
/// 
/// Struct for the JSON that will be read
#[derive(Deserialize)]
pub struct Flags {
    pub id: String,
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub btn1_label: Option<String>,
    pub btn1_url: Option<String>,
    pub btn2_label: Option<String>,
    pub btn2_url: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>
}

/// File Arg
/// 
/// Argument to use a JSON file instead to provide the Rich Presence data.
pub fn file() -> Arg<'static> {
    Arg::new(FILE)
    .long(FILE)
    .short('F')
    .takes_value(true)
    .value_name("FILE")
    .help("Uses given JSON file that provides the Rich Presence data and App ID")
    .action(ArgAction::Set)
}
pub const FILE: &str = "file";

/// ID Arg
/// 
/// Provides the Application ID
pub fn id() -> Arg<'static> {
    Arg::new(ID)
    .long(ID)
    .takes_value(true)
    .value_name("ID")
    .help("Provides the Application ID")
}
pub const ID: &str = "appid";

/// Details Arg
/// 
/// Provides the Rich Presence Details.
pub fn details() -> Arg<'static> {
    Arg::new(DETAILS)
    .long(DETAILS)
    .takes_value(true)
    .value_name("DETAILS")
    .help("Provides the Rich Presence details")
}
pub const DETAILS: &str = "details";

/// State Arg
/// 
/// Provides the Rich Presence State.
pub fn state() -> Arg<'static> {
    Arg::new(STATE)
    .long(STATE)
    .takes_value(true)
    .value_name("STATE")
    .help("Provides the Rich Presence state")
}
pub const STATE: &str = "state";

/// Large Image Arg
/// 
/// Sets the large image.
pub fn large_image() -> Arg<'static> {
    Arg::new(LARGE_IMAGE)
    .long(LARGE_IMAGE)
    .takes_value(true)
    .value_name("IMAGE")
    .help("Sets the large image")
}
pub const LARGE_IMAGE: &str = "large_image";

/// Large Image Text Arg
/// 
/// Sets the large image.
pub fn large_text() -> Arg<'static> {
    Arg::new(LARGE_TEXT)
    .long(LARGE_TEXT)
    .takes_value(true)
    .value_name("IMAGE")
    .help("Sets the large image's text")
}
pub const LARGE_TEXT: &str = "large_text";

/// Small Image Arg
/// 
/// Sets the large image.
pub fn small_image() -> Arg<'static> {
    Arg::new(SMALL_IMAGE)
    .long(SMALL_IMAGE)
    .takes_value(true)
    .value_name("IMAGE")
    .help("Sets the small image")
}
pub const SMALL_IMAGE: &str = "small_image";

/// Small Image Text Arg
/// 
/// Sets the large image.
pub fn small_text() -> Arg<'static> {
    Arg::new(SMALL_TEXT)
    .long(SMALL_TEXT)
    .takes_value(true)
    .value_name("IMAGE")
    .help("Sets the small image's text")
}
pub const SMALL_TEXT: &str = "small_text";

/// Button 1 Label
/// 
/// Sets the first button's text
pub fn btn1_label() -> Arg<'static> {
    Arg::new(BTN1_LABEL)
    .long(BTN1_LABEL)
    .takes_value(true)
    .value_name("LABEL")
    .help("Sets the first button's text")
}
pub const BTN1_LABEL: &str = "btn1_label";

/// Button 1 URL
/// 
/// Sets the first button's text
pub fn btn1_url() -> Arg<'static> {
    Arg::new(BTN1_URL)
    .long(BTN1_URL)
    .takes_value(true)
    .value_name("URL")
    .help("Sets the first button's URL")
}
pub const BTN1_URL: &str = "btn1_url";

/// Button 2 Label
/// 
/// Sets the second button's text
pub fn btn2_label() -> Arg<'static> {
    Arg::new(BTN2_LABEL)
    .long(BTN2_LABEL)
    .takes_value(true)
    .value_name("LABEL")
    .help("Sets the second button's text")
}
pub const BTN2_LABEL: &str = "btn2_label";

/// Button 2 URL
/// 
/// Sets the second button's text
pub fn btn2_url() -> Arg<'static> {
    Arg::new(BTN2_URL)
    .long(BTN2_URL)
    .takes_value(true)
    .value_name("URL")
    .help("Sets the second button's URL")
}
pub const BTN2_URL: &str = "btn2_url";

/// Start time
/// 
/// Sets the start time.
/// If it fails to parse a Unix Timestamp, it will create one from the current time.
pub fn start() -> Arg<'static> {
    Arg::new(START)
    .long("start")
    .takes_value(true)
    .value_name("UNIXTIME")
    .help("Sets the start time")
}
pub const START: &str = "start_time";

/// End time
/// 
/// Sets the end time.
pub fn end() -> Arg<'static> {
    Arg::new(END)
    .long("end")
    .takes_value(true)
    .value_name("UNIXTIME")
    .help("Sets the end time")
}
pub const END: &str = "end_time";