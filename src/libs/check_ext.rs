use std::fs::read_to_string;

// Check if line contains magic number.
pub fn check_ext(filename: &str, hex_line:&str, ext_file:&str) {
    for line in read_to_string(ext_file).unwrap().lines() {
        let split:Vec<_> = line.split("|").collect();
        if hex_line.contains(&split[0]) {
            let output = "
            -------------------------------------------------------------	
File:          ".to_owned()+filename+"
Extension(s):  "+&split[0]+"
Hex signature: "+&split[1]+"
Description:   "+&split[2]+"
-------------------------------------------------------------";
            println!("{}",output);
            break;
        }
    }
}
