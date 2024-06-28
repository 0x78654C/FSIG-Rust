use std::fs::read_to_string;
use termcolor::Color;
#[path="color_write.rs"]
mod color_write_lib;
use crate::color_write_lib::write_color;

// Check if line contains magic number.
pub fn check_ext(filename: &str, hex_line:&str, ext_file:&str, ext_only:bool) {
    for line in read_to_string(ext_file).unwrap().lines() {
        let split:Vec<_> = line.split("|").collect();
        let split_low =&split[0].to_ascii_lowercase(); 
        if hex_line.to_ascii_lowercase().starts_with(split_low) {
            let output:String;
            if ext_only {
                output = split[1].to_string();
            }else{
                output = "
    -------------------------------------------------------------	
    File:          ".to_owned()+filename+"
    Extension(s):  "+&split[1]+"
    Hex signature: "+&split[0]+"
    Description:   "+&split[2]+"
    -------------------------------------------------------------";
            }
            if output.len() < 2{
                write_color("File signature not found for this type!".to_owned(),true,Color::Yellow);  
            }
            println!("{}",output);
            break;
        }
    }
}
