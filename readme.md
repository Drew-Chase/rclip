# R-CLIP Documentation

R-CLIP (Rust Command Line Interface Parser) is a command-line parsing library for Rust. It allows you to easily define and manage command-line options and arguments for your Rust applications. This documentation will guide you through the usage of R-CLIP with examples and explanations.

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
    - [Installation](#installation)
    - [Importing R-CLIP](#importing-r-clip)
3. [Defining Command-Line Options](#defining-command-line-options)
    - [Creating Option Objects](#creating-option-objects)
    - [Initializing OptionsManager](#initializing-optionsmanager)
4. [Parsing Command-Line Arguments](#parsing-command-line-arguments)
    - [Parsing Arguments](#parsing-arguments)
    - [Accessing Option Values](#accessing-option-values)
5. [Example](#example)

---

## 1. Introduction <a name="introduction"></a>

R-CLIP is a Rust library that simplifies the process of parsing command-line arguments and options for your applications. With R-CLIP, you can define the options your program supports, parse command-line arguments, and access the values of specified options.

## 2. Getting Started <a name="getting-started"></a>

### Installation <a name="installation"></a>

To use R-CLIP in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
rclip = "0.0.1"
```

### Importing R-CLIP <a name="importing-r-clip"></a>

In your Rust code, import R-CLIP as follows:

```rust
extern crate rclip;
use rclip::{Option, OptionsManager};
```

## 3. Defining Command-Line Options <a name="defining-command-line-options"></a>

### Creating Option Objects <a name="creating-option-objects"></a>

You can define command-line options using the `Option` struct from R-CLIP. An option is defined by its short name, long name, whether it is required, whether it expects an argument, and a description.

Example option definitions:

```rust
let option1 = Option::new("v", "version", false, false, "Prints version information");
let option2 = Option::new("f", "file", true, true, "The file to read");
let option3 = Option::new("o", "output", false, true, "The file to write to");
```

### Initializing OptionsManager <a name="initializing-optionsmanager"></a>

After defining your options, you need to create an `OptionsManager` to manage and parse them. The `OptionsManager` constructor takes the application name and a vector of `Option` objects as parameters.

```rust
let options = vec![option1, option2, option3];
let mut options_manager = OptionsManager::new("MyApp", options);
```

## 4. Parsing Command-Line Arguments <a name="parsing-command-line-arguments"></a>

### Parsing Arguments <a name="parsing-arguments"></a>

You can parse command-line arguments using the `parse_options` method of the `OptionsManager`:

```rust
let args = vec!["-h", "-o", "output.txt"];
let options_result = options_manager.parse_options(args);
```

### Accessing Option Values <a name="accessing-option-values"></a>

After parsing, you can check whether specific options were present and access their argument values if applicable:

```rust
if options_result.is_ok() {
    if options_manager.is_present("h") {
        println!("Help is present");
    }
    if options_manager.is_present("o") {
        println!("Output file: {}", options_manager.argument("o").unwrap());
    }
} else {
    println!("Error: {}", options_result.err().unwrap());
    options_manager.print_help();
}
```

## 5. Example <a name="example"></a>

Here's an example of using R-CLIP to define and parse command-line options in Rust:

```rust
extern crate rclip;
use rclip::{Option, OptionsManager};

fn main() {
    let option1 = Option::new("v", "version", false, false, "Prints version information");
    let option2 = Option::new("f", "file", true, true, "The file to read");
    let option3 = Option::new("o", "output", false, true, "The file to write to");

    let options = vec![option1, option2, option3];
    let mut options_manager = OptionsManager::new("MyApp", options);

    let args = vec!["-h", "-o", "output.txt"];
    let options_result = options_manager.parse_options(args);

    if options_result.is_ok() {
        if options_manager.is_option_present("h") {
            println!("Help is present");
        }
        if options_manager.is_option_present("o") {
            println!("Output file: {}", options_manager.argument("o").unwrap());
        }
    } else {
        println!("Error: {}", options_result.err().unwrap());
        options_manager.print_help();
    }
}
```

This is a basic example to get you started with R-CLIP. You can customize it for your specific application and use case.

---

That's it! You've now learned how to use R-CLIP to parse command-line arguments and options in Rust. Feel free to explore more advanced features and customization options provided by R-CLIP in the official documentation.