//! # Option
//!
//! The `Option` struct represents a command-line option that can be used with an application.
//!
//! ## Usage
//!
//! ```rust
//! use crate::option::Option;
//!
//! // Create an Option instance
//! let option = Option::new("v".to_string(), "version".to_string(), false, false, "Prints version information".to_string());
//!
//! // Access the properties of the Option
//! println!("Short Name: {}", option.short_name);
//! println!("Long Name: {}", option.long_name);
//! println!("Required: {}", option.required);
//! println!("Has Argument: {}", option.has_argument);
//! println!("Description: {}", option.description);
//! ```
//!
//! ## Examples
//!
//! ### Creating an Option Instance
//!
//! ```rust
//! use crate::option::Option;
//!
//! // Create a new Option instance
//! let option = Option::new("v".to_string(), "version".to_string(), false, false, "Prints version information".to_string());
//! ```
//!
//! ### Getting Option Properties
//!
//! ```rust
//! use crate::option::Option;
//!
//! let option = Option::new("v".to_string(), "version".to_string(), false, false, "Prints version information".to_string());
//!
//! println!("Short Name: {}", option.short_name);
//! println!("Long Name: {}", option.long_name);
//! println!("Required: {}", option.required);
//! println!("Has Argument: {}", option.has_argument);
//! println!("Description: {}", option.description);
//! ```
//!
//! ## License
//!
//! This code is licensed under the MIT License.
//!
//! ## Disclaimer
//!
//! This code is provided as-is, and the authors make no warranties regarding its correctness or fitness for any purpose.
//!
//! Please feel free to report issues or submit pull requests to improve this code.
//!

/// The Option struct represents a command line option.
#[derive(Clone)]
pub struct Option {
    /// The short name of the option, e.g., "h" for "-h".
    pub short_name: String,
    /// The long name of the option, e.g., "help" for "--help".
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
    /// Creates a new Option instance with the specified properties.
    ///
    /// # Arguments
    ///
    /// * `short_name` - A string representing the short name of the option.
    /// * `long_name` - A string representing the long name of the option.
    /// * `required` - A boolean indicating whether the option is required.
    /// * `has_argument` - A boolean indicating whether the option has an argument.
    /// * `description` - A string representing the description of the option.
    ///
    /// # Returns
    ///
    /// A new `Option` instance.
    pub fn new(short_name: String, long_name: String, required: bool, has_argument: bool, description: String) -> Self {
        Self { short_name, long_name, required, has_argument, description, argument: String::new() }
    }

    /// Returns a string representation of the Option struct.
    ///
    /// # Returns
    ///
    /// A formatted string containing the option's properties.
    pub fn to_string(&self) -> String {
        format!(
            "short:{}, long:{}, required:{}, has argument:{}, description:{}",
            self.short_name, self.long_name, self.required, self.has_argument, self.description
        )
    }
}
