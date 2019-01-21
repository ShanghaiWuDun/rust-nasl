#[macro_use]
extern crate log;
extern crate env_logger;

extern crate libc;
extern crate nasl_sys;

use std::env;
use std::ptr;
use std::mem;
use std::ffi::{ CStr, CString };


pub fn version() -> String {
    unsafe {
        CStr::from_ptr(nasl_sys::nasl_version())
            .to_str().unwrap().to_string()
    }
}



pub unsafe fn run_nasl() -> Result<(), ()> {
    nasl_sys::gcrypt_init();
    nasl_sys::openvas_SSL_init();

    let ret_code = nasl_sys::add_nasl_inc_dir(CString::new("/Users/luozijun/Project/diting/openvas-data/nvt-feed").unwrap().into_raw());
    assert_eq!(ret_code, 0);
    let ret_code = nasl_sys::add_nasl_inc_dir(CString::new("/Users/luozijun/Project/diting/rust-nasl").unwrap().into_raw());
    assert_eq!(ret_code, 0);

    let hosts = nasl_sys::gvm_hosts_new(CString::new("127.0.0.1").unwrap().into_raw());
    
    debug!("hosts: {:?}", hosts);

    nasl_sys::gvm_hosts_resolve(hosts);

    loop {
        let host = nasl_sys::gvm_hosts_next (hosts);
        
        if host.is_null() {
            break;
        }

        debug!("host: {:?}", host);

        let mut ip6: libc::in6_addr = mem::zeroed();
        nasl_sys::gvm_host_get_addr6(host, &mut ip6);


        let vclass = *nasl_sys::KBDefaultOperations;

        assert_eq!(vclass.kb_new.is_some(), true);
        let kb_new = vclass.kb_new.unwrap();

        let mut kb: nasl_sys::kb_t = ptr::null_mut();

        let ret_code = kb_new(&mut kb, CString::new(nasl_sys::KB_PATH_DEFAULT).unwrap().into_raw());
        if ret_code > 0 {
            error!("kb_new failed. ret code: {:?}", ret_code);
            return Err(());
        }
        
        let mut script_infos = nasl_sys::init(&mut ip6, (*host).vhosts, kb);
        debug!("script_infos: {:?}", script_infos);
        
        (*script_infos).name = CString::new("ssh_detect.nasl").unwrap().into_raw();

        let nvti: *mut nasl_sys::nvti_t = nasl_sys::parse_script_infos(script_infos);
        if nvti.is_null() {
            error!("parse_script_infos failed.");
            return Err(());
        }

        let mode = 0 | nasl_sys::NASL_COMMAND_LINE;

        let ret_code = nasl_sys::exec_nasl_script(script_infos, mode);
        if ret_code != 0 {
            error!("exec_nasl_script ret code: {:?}", ret_code);
            return Err(());
        }
    }

    Ok(())
}

fn main() {
    env::set_var("RUST_LOG", "info");

    env_logger::init();

    info!("nasl version: {:?}", version());
    
    unsafe {
        let _ = run_nasl();
    }
}
