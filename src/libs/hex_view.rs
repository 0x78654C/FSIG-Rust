pub fn get_hex_rep(byte_array: &mut [u8]) -> String {
    let build_string_vec: Vec<String> = byte_array.chunks(2)
        .map(|c| {
            if c.len() == 2 { format!("{:02x} {:02x}", c[0], c[1]) }
            else { format!("{:02x}", c[0]) }
        }).collect();

    build_string_vec.join(" ")
}