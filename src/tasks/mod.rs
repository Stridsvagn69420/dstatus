pub struct Data {
    pub id: String,
    pub state: Option<String>,
    pub details: Option<String>
}

impl Data {
    pub fn empty() -> Data {
        Data {
            id: "".to_string(),
            state: None,
            details: None
        }
    }
}