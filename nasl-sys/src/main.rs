
use std::os::raw::{c_char, };

#[link(name = "nasl", kind = "dylib")]
extern {
    pub fn nasl_version() -> *mut c_char;
}


fn main() {
    unsafe {
        println!("nasl version: {:?}", nasl_version());
    }
}
