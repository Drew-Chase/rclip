//! # OptionsManager
//!
//! The `OptionsManager` struct represents a command line option manager.
//!
//! ## Usage
//!
//! ```rust
//! use std::collections::HashSet;
//! use crate::option::Option;
//!
//! let mut options_manager = OptionsManager::new("MyApp", Vec::new());
//! let args = vec!["-h", "-o", "output.txt"];
//!
//! if let Ok(present_options) = options_manager.parse_options(args) {
//!     if options_manager.is_present("h") {
//!         options_manager.print_help();
//!     } else {
//!         for option in &options_manager.options {
//!             // Check if the option is present
//!             if options_manager.is_present(&option.short_name) {
//!                 let argument = options_manager.argument(&option.short_name);
//!                 // Process the option and its argument
//!                 println!("Option: -{}, Argument: {}", option.short_name, argument);
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Examples
//!
//! ### Creating an `OptionsManager` Instance
//!
//! ```rust
//! use rclip::{Option, OptionsManager};
//!
//! // Create a new OptionsManager instance with default "help" option
//! let options = vec![
//!     Option::new("v".to_string(), "version".to_string(), false, false, "Prints version information".to_string()),
//!     Option::new("f".to_string(), "file".to_string(), true, true, "The file to read".to_string()),
//!     Option::new("o".to_string(), "output".to_string(), false, true, "The file to write to".to_string()),
//! ];
//!
//! let mut options_manager = OptionsManager::new("MyApp", options);
//! ```
//!
//! ### Parsing Command-Line Options
//!
//! ```rust
//! let args = vec!["-h", "-o", "output.txt"];
//! if let Ok(present_options) = options_manager.parse_options(args) {
//!     // Check if the "help" option is present
//!     if options_manager.is_present("h") {
//!         options_manager.print_help();
//!     } else {
//!         for option in &options_manager.options {
//!             // Check if the option is present
//!             if options_manager.is_present(&option.short_name) {
//!                 let argument = options_manager.argument(&option.short_name);
//!                 // Process the option and its argument
//!                 println!("Option: -{}, Argument: {}", option.short_name, argument);
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ### Checking for Required Options
//!
//! ```rust
//! let args = vec!["-f", "input.txt"];
//! if let Ok(present_options) = options_manager.parse_options(args) {
//!     for option in &options_manager.options {
//!         // Check if a required option is missing
//!         if option.required && !options_manager.is_present(&option.short_name) {
//!             eprintln!("Required option -{} or --{} is missing.", option.short_name, option.long_name);
//!         }
//!     }
//! }
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

use std::collections::HashSet;
use crate::option::Option;

/// The OptionsManager struct represents a command line option manager.
pub struct OptionsManager {
    context: String,
    options: Vec<Option>,
    present_options: Vec<Option>,
}
#[warn(dead_code)]
/// `OptionsManager` implementation.
impl OptionsManager {
    /// Instantiates a new `OptionsManager` instance.
    ///
    /// # Arguments
    ///
    /// * `context` - A string representing the application context.
    /// * `options` - A vector of `Option` objects representing the available command-line options.
    ///
    /// # Returns
    ///
    /// A new `OptionsManager` instance.
    pub fn new(context: &str, mut options: Vec<Option>) -> Self {
        options.push(Option::new("h".to_string(), "help".to_string(), false, false, "Displays the help screen".to_string()));
        Self { context: context.to_string(), options, present_options: Vec::new() }
    }

    /// Parses the command line options and returns option that are present.
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of strings representing the command-line arguments.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Option` objects representing the parsed options if successful,
    /// or an error message as a string if parsing fails.
    pub fn parse_options(&mut self, args: Vec<String>) -> Result<Vec<Option>, String> {
        let mut present_options: Vec<Option> = Vec::new();
        let mut option_set: HashSet<&str> = HashSet::new();
        let prefix = ["-", "--"];

        for option in &self.options {
            option_set.insert(&option.short_name);
            option_set.insert(&option.long_name);
        }

        let mut index = 0;
        while index < args.len() {
            let arg = &args[index];
            if prefix.contains(&&arg[..1]) {
                if let Some(option) = option_set.get(&arg[1..]) {
                    let mut parsed_option = Option::new(
                        option.to_string(),
                        option.to_string(),
                        false, // Initialize required as false
                        false, // Initialize has_argument as false
                        "".to_string(),
                    );

                    if let Some(tmp) = self.options.iter().find(|o| o.short_name == *option || o.long_name == *option) {
                        parsed_option.required = tmp.required;
                        parsed_option.has_argument = tmp.has_argument;
                        parsed_option.description = tmp.description.clone();
                    }

                    if parsed_option.has_argument {
                        index += 1;
                        if index < args.len() {
                            parsed_option.argument = args[index].clone();
                        } else {
                            return Err(format!("Option {} requires an argument.", arg));
                        }
                    }
                    present_options.push(parsed_option);
                } else {
                    return Err(format!("Unknown option: {}", arg));
                }
            }
            index += 1;
        }

        self.present_options = present_options.clone();

        if self.is_present("h") {
            self.print_help();
        } else {
            for option in &self.options {
                if option.required && !present_options.iter().any(|o| o.short_name == option.short_name || o.long_name == option.long_name) {
                    return Err(format!("Required option -{} or --{} is missing.", option.short_name, option.long_name));
                }
            }
        }
        return Ok(present_options);
    }

    /// Checks if the given option is present in the parsed options list.
    ///
    /// # Arguments
    ///
    /// * `option` - A string representing the option to check.
    ///
    /// # Returns
    ///
    /// `true` if the option is present; otherwise, `false`.
    pub fn is_present(&self, option: &str) -> bool {
        return self.present_options.iter().any(|o| o.short_name == option || o.long_name == option);
    }

    /// Returns the argument of the given option.
    ///
    /// # Arguments
    ///
    /// * `option` - A string representing the option to retrieve the argument for.
    ///
    /// # Returns
    ///
    /// The argument of the option as a string.
    pub fn argument(&self, option: &str) -> String {
        let mut argument = String::new();
        if let Some(option) = self.present_options.iter().find(|o| o.has_argument && (o.short_name == option || o.long_name == option)) {
            argument = option.argument.clone();
        }
        return argument;
    }

    /// Prints the help screen.
    pub fn print_help(&self) {
        println!("{} - Help: ", self.context);
        for option in &self.options {
            println!("  -{}, --{}{}{}", option.short_name, option.long_name, if option.has_argument { " <arg>" } else { "" }, if option.required { " (required)" } else { "" });
            println!("      {}", option.description);
        }
    }
}
    