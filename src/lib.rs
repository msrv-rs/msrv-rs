use toml::value::Array;

#[no_mangle]
pub extern fn plugin_query(ctx: *mut Plugin, s: * const u8, len: usize) -> u32 {
    let mut ctx = unsafe {
        Box::from_raw(ctx)
    };
    let s = unsafe {
        let sl = std::slice::from_raw_parts(s, len);
        std::str::from_utf8(sl)
    };
    let s = match s {
        Ok(s) => s,
        // Error
        Err(_) => return u32::max_value(),
    };
    ctx.handle_line(s)
}

#[no_mangle]
pub extern fn plugin_init(s: * const u8, len: usize) -> * mut Plugin {
    let _params = unsafe {
        let sl = std::slice::from_raw_parts(s, len);
        std::str::from_utf8(sl)
    };
    let ctx = Box::new(Plugin::new(Vec::new()));
    Box::into_raw(ctx)
}

#[no_mangle]
pub extern fn plugin_delete(ctx: * mut Plugin) {
    // Obtain the Box and drop it
    let _b = unsafe {
        Box::from_raw(ctx)
    };
}

pub struct Plugin {
    excludes: Array,
}

impl Plugin {
    fn new(excludes: Array) -> Self {
        Self {
            excludes,
        }
    }
    fn handle_line(&mut self, line: &str) -> u32 {
        if line.contains("adler32 1.0.4") || line.contains("adler32 1.0.3") {
            0
        } else {
            1
        }
    }
}
