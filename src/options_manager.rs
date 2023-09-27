use std::collections::HashSet;
use crate::option::Option;

/// The OptionsManager struct represents a command line option manager.
pub struct OptionsManager {
    context: String,
    options: Vec<Option>,
    present_options: Vec<Option>,
}

/// `OptionsManager` implementation.
impl OptionsManager {
    /// Instantiates a new `OptionsManager` instance.
    pub fn new(context: &str, mut options: Vec<Option>) -> Self {
        options.push(Option::new("h".to_string(), "help".to_string(), false, false, "Displays the help screen".to_string()));
        Self { context: context.to_string(), options, present_options: Vec::new() }
    }

    /// Parses the command line options and returns option that are present.
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
    pub fn is_present(&self, option: &str) -> bool {
        return self.present_options.iter().any(|o| o.short_name == option || o.long_name == option);
    }

    /// Returns the argument of the given option.
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
    