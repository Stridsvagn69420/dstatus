use discord_rich_presence::activity::{Activity, Assets, Button, Timestamps};

/// Bundled Rich Presence Data
pub struct Data {
    pub id: String,
    pub state: Option<String>,
    pub details: Option<String>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub btn1_label: Option<String>,
    pub btn1_url: Option<String>,
    pub btn2_label: Option<String>,
    pub btn2_url: Option<String>,
    pub start: Option<i64>,
    pub end: Option<i64>,
}

impl Data {
    /// Parameters to [Data]
    ///
    /// Create a new [Data] with set parameters.
    /// Basically just a function to make a new [Data].
    pub fn new(
        id: String,
        state: Option<String>,
        details: Option<String>,
        large_image: Option<String>,
        large_text: Option<String>,
        small_image: Option<String>,
        small_text: Option<String>,
        btn1_label: Option<String>,
        btn1_url: Option<String>,
        btn2_label: Option<String>,
        btn2_url: Option<String>,
        start: Option<i64>,
        end: Option<i64>,
    ) -> Data {
        Data {
            id,
            state,
            details,
            large_image,
            large_text,
            small_image,
            small_text,
            btn1_label,
            btn1_url,
            btn2_label,
            btn2_url,
            start,
            end,
        }
    }

    /// Dummy [Data]
    ///
    /// Creates an empty [Data] as a dummy
    pub fn empty() -> Data {
        Data {
            id: "".to_string(),
            state: None,
            details: None,
            large_image: None,
            large_text: None,
            small_image: None,
            small_text: None,
            btn1_label: None,
            btn1_url: None,
            btn2_label: None,
            btn2_url: None,
            start: None,
            end: None,
        }
    }

    /// Data to Activity
    ///
    /// Converts the [Data] to an [Activity]
    pub fn to_activity(&self) -> Activity {
        activity(
            &self.state,
            &self.details,
            &self.large_image,
            &self.large_text,
            &self.small_image,
            &self.small_text,
            &self.btn1_label,
            &self.btn1_url,
            &self.btn2_label,
            &self.btn2_url,
            &self.start,
            &self.end
        )
    }
}

/// Parameters to [Activity]
///
/// Accepts [Data]'s parameters to create a new [Activity]
pub fn activity<'a>(
    state: &'a Option<String>,
    details: &'a Option<String>,
    large_image: &'a Option<String>,
    large_text: &'a Option<String>,
    small_image: &'a Option<String>,
    small_text: &'a Option<String>,
    btn1_label: &'a Option<String>,
    btn1_url: &'a Option<String>,
    btn2_label: &'a Option<String>,
    btn2_url: &'a Option<String>,
    start: &'a Option<i64>,
    end: &'a Option<i64>
) -> Activity<'a> {
    // Create Activity
    let mut act = Activity::new();
    let mut assets = Assets::new();
    let mut time = Timestamps::new();
    let mut buttons: Vec<Button> = Vec::new();

    // Set state
    if let Some(x) = state {
        act = act.state(x);
    }

    // Set details
    if let Some(x) = details {
        act = act.details(x);
    }

    // Set large_image
    if let Some(x) = large_image {
        assets = assets.large_image(x);
    }

    // Set large_text
    if let Some(x) = large_text {
        assets = assets.large_text(x);
    }

    // Set small_image
    if let Some(x) = small_image {
        assets = assets.small_image(x);
    }

    // Set small_text
    if let Some(x) = small_text {
        assets = assets.small_text(x);
    }

    // Set button 1
    if let Some(x) = btn1_label {
        if let Some(y) = btn1_url {
            buttons.push(Button::new(x, y));
        } else {
            buttons.push(Button::new(x, x));
        }
    }

    // Set button 2
    if let Some(x) = btn2_label {
        if let Some(y) = btn2_url {
            buttons.push(Button::new(x, y));
        } else {
            buttons.push(Button::new(x, x));
        }
    }

    // Set start time
    if let Some(x) = start {
        time = time.start(*x);
    }

    // Set end time
    if let Some(x) = end {
        time = time.end(*x);
    }

    // Create final Activity
    act.assets(assets).timestamps(time).buttons(buttons)
}
