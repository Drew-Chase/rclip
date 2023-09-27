#[derive(Clone)]
/// The Option struct represents a command line option.
pub struct Option {
    /// The short name of the option, e.g. "h" for "-h".
    pub short_name: String,
    /// The long name of the option, e.g. "help" for "--help".
    pub long_name: String,
    /// Whether the option is required or not.
    pub required: bool,
    /// Whether the option has an argument or not.
    pub has_argument: bool,
    /// The description of the option.
    pub description: String,
    /// The argument of the option.
    pub argument: String,
}

/// The implementation of the Option struct.
impl Option {
    pub fn new(short_name: String, long_name: String, required: bool, has_argument: bool, description: String) -> Self {
        Self { short_name, long_name, required, has_argument, description, argument: String::new() }
    }
    /// Returns the string representation of the Option struct.
    pub fn to_string(&self) -> String {
        return format!("short:{}, long:{}, required:{}, has argument:{}, description:{}", self.short_name, self.long_name, self.required, self.has_argument, self.description);
    }
}