use std::env;
use std::path::Path;
use termcolor::Color;
use std::fs::File;
use std::io::Read;

const GLOBAL_BUFFER_LENGTH: usize = 16;

#[path="./libs/color_write.rs"]
mod color_write_lib;
use crate::color_write_lib::write_color;

#[path="./libs/hex_view.rs"]
mod hex_view_lib;
use crate::hex_view_lib::get_hex_rep;

#[path="./libs/check_ext.rs"]
mod check_ext_lib;
use crate::check_ext_lib::check_ext;

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
        return;
    }

    // Check if ext file exist.
    let ext_file_exist: bool = Path::new(EXT_FILE).is_file();
    if !ext_file_exist {
        write_color("Error: ext_list.txt does not exist!".to_owned(),true,Color::Red);  
        return;
    }

    // Check if argument file exist.
    let exist_file_from_arg : bool =  Path::new(arg1).is_file();
    if !exist_file_from_arg {
        write_color("Error: File '".to_owned()+ arg1 +"' does not exist!",true,Color::Red);  
        return;
    }

    // Read file 
    let mut file_to_read = get_file(String::from(arg1));

    let mut buff = [0; GLOBAL_BUFFER_LENGTH];
    let mut count_loops:i32 =0;
    let mut hex_line:String = "".to_string();

    loop {
        count_loops += 1;
        let bytes_read = file_to_read.read(&mut buff);
        match bytes_read {
            Ok(number) => {
                if number == 0 {
                    break;
                } else {
                    
                    // Check if hex contains magic number from list.
                     let get_hex = get_hex_rep(&mut buff[0..number]);
                     hex_line += &get_hex;

                    // Break loop after fist 5 lines.
                    if count_loops == 5{
                        break;
                    }
                }
            },
            Err(why) => {
                eprintln!("rhexdump: {}", why);
                break;
            }
        }
    }

    // Print the final result;
    check_ext(arg1, hex_line.as_str(), EXT_FILE);
}

// Open file.
fn get_file(path_to_file: String) -> File {
    match File::open(path_to_file) {
        Ok(f) => File::from(f),
        Err(e) => {
            panic!("{}", e);
        }
    }
}