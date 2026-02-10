use std::ffi::{c_char, CStr};

pub const VERSION_STR: &str = "Stupid Network Library v0.1.0";

pub fn get_version_rust() -> &'static str {
    VERSION_STR
}

const VERSION_C: &[u8] = b"Stupid Network Library v0.1.0\0";

#[unsafe(no_mangle)]
pub extern "C" fn net_get_version() -> *const c_char {
    VERSION_C.as_ptr() as *const c_char
}