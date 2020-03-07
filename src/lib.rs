#[no_mangle]
pub extern fn query(s: * const u8, len: usize) -> u32 {
    let s = unsafe {
        let sl = std::slice::from_raw_parts(s, len);
        std::str::from_utf8(sl)
    };
    let s = match s {
        Ok(s) => s,
        // Error
        Err(_) => return u32::max_value(),
    };
    handle_line(s)
}

fn handle_line(line: &str) -> u32 {
    if line.contains("adler32 1.0.4") {
        0
    } else {
        1
    }
}
