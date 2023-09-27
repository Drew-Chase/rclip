mod option;
mod options_manager;

#[cfg(test)]
#[warn(dead_code)]
mod tests {
    use crate::options_manager::OptionsManager;
    use crate::option::Option;

    #[test]
    fn it_works() {
        let test_options: Vec<Option> = vec![
            Option::new("v".to_string(), "version".to_string(), false, false, "Prints version information".to_string()),
            Option::new("f".to_string(), "file".to_string(), true, true, "The file to read".to_string()),
            Option::new("o".to_string(), "output".to_string(), false, true, "The file to write to".to_string()),
        ];
        let mut options_manager = OptionsManager::new("R-CLIP (Rust Command Line Interface Parser)", test_options);
        let options_result = options_manager.parse_options(vec!["-h".to_string(), "-o".to_string(), "output.txt".to_string()]);

        if options_result.is_ok() {
            if options_manager.is_present("h") {
                println!("Help is present");
            }
            if options_manager.is_present("o") {
                println!("Output file: {}", options_manager.argument("o"));
            }
        } else {
            println!("Error: {}", options_result.err().unwrap());
            options_manager.print_help();
        }
    }
}
