use toml::Value;
use toml::value::Array;

#[no_mangle]
pub extern fn plugin_query(ctx: &mut Plugin, s: * const u8, len: usize) -> u32 {
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
pub extern fn plugin_init(s: * const u8, len: usize) -> Box<Plugin> {
    let params_str = unsafe {
        let sl = std::slice::from_raw_parts(s, len);
        std::str::from_utf8(sl)
    };
    let excludes = get_excludes(params_str.ok())
        .unwrap_or_else(|| Vec::new());
    Box::new(Plugin::new(excludes))
}

#[no_mangle]
pub extern fn plugin_delete(ctx: Box<Plugin>) {
    // The call isn't needed but it makes the intent
    // of dropping explicit
    std::mem::drop(ctx);
}

fn get_excludes(params_str: Option<&str>) -> Option<Array> {
    let params: Value = toml::from_str(params_str?).ok()?;
    let excludes = params.get("exclude")?.as_array()?.clone();
    Some(excludes)
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
        for exclude in self.excludes.iter() {
            if let Some(exclude) = exclude.as_str() {
                if line.contains(exclude) {
                    return 0;
                }
            }
        }
        1
    }
}
