#[macro_use]
extern crate log;
extern crate nasl_sys;


use std::ffi::CStr;


fn main() {
    unsafe {
        let version = CStr::from_ptr(nasl_sys::nasl_version());
        println!("nasl version: {:?}", version);
    }
}
