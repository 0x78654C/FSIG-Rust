use std::{env, io};
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[path="./libs/color_write.rs"]
mod color_write_lib;

use crate::color_write_lib::write_color;


const EXT_FILE:&str = "ext_list.txt";
const HELP_MESSAGE: &str = "\
Simple CLI application for file type (signature) check by magical numbers.
Usage of file extension tool:
 	FSIG-Rust <file_path>      : Display file path, extension, hex signature, and signature description.
 	FSIG-Rust <file_path> -ext : Display extension only.
 	FSIG-Rust -h               : Display this help message.

Author: x_Coding (xcoding.dev@gmail.com)
";

// Entry point function.
fn main() {
    let args: Vec<String> = env::args().collect();

    // In case of no argument is passed to application.
    if args.len() == 1 {
        println!("{}","Use -h to list the parameters!\n");
        return;
    }

    // Read first parameter.
    let arg1 = &args[1];

    // Display help message.
    if arg1 == "-h" {
        println!("{}", HELP_MESSAGE);
    }

    // Check if ext file exist.
    let ext_file_exist: bool = Path::new(EXT_FILE).is_file();
    if !ext_file_exist {
        write_color("Error: ext_list.txt does not exist!".to_owned(),true,Color::Red);  
        return;
    }
}