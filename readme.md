# RCLIP-CMD (Rust Command Line Interface Parser)

**RCLIP-CMD** (Rust Command Line Interface Parser) is a Rust library designed to simplify the process of parsing command-line arguments and options for your Rust applications. With RCLIP-CMD, you can easily define and manage command-line options, parse arguments, and access the values of specified options.

**Table of Contents**
1. [Features](#features)
2. [Getting Started](#getting-started)
   - [Add RCLIP-CMD to Your Dependencies](#1-add-rclip-cmd-to-your-dependencies)
   - [Import Necessary Modules](#2-import-necessary-modules)
   - [Define Your Command-Line Options](#3-define-your-command-line-options)
3. [Example](#example)
   - [Defining Command-Line Options](#1-defining-command-line-options)
   - [Creating an `OptionsManager` Instance](#2-creating-an-optionsmanager-instance)
   - [Parsing Command-Line Arguments](#3-parsing-command-line-arguments)
   - [Checking if Options Are Present](#4-checking-if-options-are-present)
   - [Handling Errors](#5-handling-errors)
4. [License](#license)
5. [Disclaimer](#disclaimer)

## 1. Features <a name="features"></a>

- Define and manage command-line options.
- Parse command-line arguments.
- Access the values of specified options.
- Print a help screen.

## 2. Getting Started <a name="getting-started"></a>

To get started with RCLIP-CMD, follow these steps:

### 1. Add RCLIP-CMD to Your Dependencies <a name="1-add-rclip-cmd-to-your-dependencies"></a>

To use RCLIP-CMD in your Rust project, add it to your `Cargo.toml` file as a dependency:

```toml
[dependencies]
rclip-cmd = "1.0.0"
```

This step ensures that your project can access the RCLIP-CMD library.

### 2. Import Necessary Modules <a name="2-import-necessary-modules"></a>

In your Rust code, import the modules and types provided by RCLIP-CMD to use its functionality:

```rust
use std::env;
use rclip_cmd::{options_manager::OptionsManager, option::Option};
```

Here, we import the necessary modules from RCLIP-CMD, including `OptionsManager` and `Option`, which are essential for defining and managing command-line options.

### 3. Define Your Command-Line Options <a name="3-define-your-command-line-options"></a>

Before you can parse command-line arguments, you need to define the options for your application using the `Option` struct from RCLIP-CMD. Each option is configured with a short name, a long name, and additional information about whether it is required and whether it expects an argument.

## 3. Example <a name="example"></a>

Here's a detailed example of how to use RCLIP-CMD to define, parse, and work with command-line options in Rust:

### 1. Defining Command-Line Options <a name="1-defining-command-line-options"></a>

In this part of the code, we define the command-line options for our application using the `Option` struct from RCLIP-CMD. Each option is configured with various properties:

```rust
let options = vec![
    Option::new("v".to_string(), "version".to_string(), false, false, "Prints version information".to_string()),
    Option::new("o".to_string(), "output".to_string(), false, false, "The output file to write to".to_string()),
    Option::new("i".to_string(), "input".to_string(), true, true, "The input file".to_string()),
];
```

- `Option::new()`: This function creates a new `Option` instance with the specified properties. It takes the short name, long name, whether it's required, whether it expects an argument, and a description as arguments.

### 2. Creating an `OptionsManager` Instance <a name="2-creating-an-optionsmanager-instance"></a>

After defining the command-line options, we create an `OptionsManager` instance to manage these options and handle argument parsing:

```rust
let mut options_manager = OptionsManager::new("Test Application", options);
```

- `OptionsManager::new()`: This constructor creates a new `OptionsManager` instance. It takes two arguments: the context (a string representing the application name) and a vector of `Option` objects representing the available command-line options.

### 3. Parsing Command-Line Arguments <a name="3-parsing-command-line-arguments"></a>

Next, we parse the command-line arguments using the `parse_options` function:

```rust
let result = options_manager.parse_options(env::args().collect());
```

- `env::args().collect()`: This collects the command-line arguments into a `Vec<String>`, which is then passed to the `parse_options` function.

- `options_manager.parse_options()`: This function parses the provided command-line arguments. It returns a `Result` where `Ok` contains a vector of `Option` objects representing the parsed options, or `Err` contains an error message as a string if parsing fails.

### 4. Checking if Options Are Present <a name="4-checking-if-options-are-present"></a>

We use various functions provided by the `OptionsManager` instance to check if specific options are present and retrieve their argument values:

```rust
if options_manager.is_present("v") {
    println!("Version 0.1.0");
}
if options_manager.is_present("i") {
    let argument = options_manager.argument("i");
    println!("Input: {}", argument);
}
if options_manager.is_present("o") {
    let argument = options_manager.argument("o");
    println!("Output: {}", argument);
}
```

- `options_manager.is_present(option_name)`: This function checks if the specified option (by short name) is present in the parsed options. It returns `true` if the option is found, otherwise `false`.

- `options_manager.argument(option_name)`: This function retrieves the argument value associated with the specified option (by short name) if the option expects an argument. If the option does not have an argument or is not present, it returns an empty string.

### 5. Handling Errors <a name="5-handling-errors"></a>

Lastly, we handle any errors that might occur during argument parsing:

```rust
else {
    println!("Error: {}", result.err().unwrap());
}
```

- If the `parse_options` function returns an error (`Err`), we print the error message to the console.

This code provides a complete example of how to define, manage, parse, and work with command-line options in Rust using the RCLIP-CMD library.

## 4. License <a name="license"></a>

This code is licensed under the GNU General Public License, version 3.0 (GPL-3.0).

## 5. Disclaimer <a name="disclaimer"></a>

This code is provided as-is, and the authors make no warranties regarding its correctness or fitness for any purpose. Please feel free to report issues or submit pull requests to improve this code.