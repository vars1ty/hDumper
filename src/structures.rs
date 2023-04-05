use tabled::Tabled;

/// Person structure.
/// If a specific value wasn't found, the fallback value for it is "???".
#[derive(Tabled, PartialEq)]
pub struct Person {
    pub name: String,
    pub gender: String,
    pub age: u16,
    #[tabled(rename = "addr")]
    pub address: String,
    #[tabled(rename = "married")]
    pub married: String,
    #[tabled(rename = "tele")]
    pub numbers: String,
    #[tabled(rename = "src")]
    pub source: String,
}

/// Default values for `Person`.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("Unknown"),
            gender: String::from("Unknown"),
            age: 0,
            address: String::from("Unknown"),
            married: String::from("?"),
            numbers: String::from("???"),
            source: String::from("???"),
        }
    }
}
