#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate libc;
extern crate nasl_sys;

use std::fs;
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


pub fn c_str<T: Into<Vec<u8>>>(t: T) -> *mut libc::c_char {
    CString::new(t).unwrap().into_raw()
}

pub unsafe fn run_nasl(filename: &str, mode: libc::c_int, target: &str) -> Result<(), ()> {
    nasl_sys::gcrypt_init();
    nasl_sys::openvas_SSL_init();

    let hosts = nasl_sys::gvm_hosts_new(c_str(target));
    
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

        let ret_code = kb_new(&mut kb, c_str(nasl_sys::KB_PATH_DEFAULT));
        if ret_code > 0 {
            error!("kb_new failed. ret code: {:?}", ret_code);
            return Err(());
        }
        
        let mut script_infos = nasl_sys::init(&mut ip6, (*host).vhosts, kb);
        debug!("script_infos: {:?}", script_infos);
        
        (*script_infos).name = c_str(filename);

        if mode == nasl_sys::NASL_EXEC_DESCR {
            let nvti: *mut nasl_sys::nvti_t = nasl_sys::parse_script_infos(script_infos);
            if nvti.is_null() {
                error!("parse_script_infos failed.");
                return Err(());
            }
        } else if mode == nasl_sys::NASL_COMMAND_LINE {
            let ret_code = nasl_sys::exec_nasl_script(script_infos, mode);
            if ret_code != 0 {
                error!("exec_nasl_script ret code: {:?}", ret_code);
                return Err(());
            }
        } else {
            unimplemented!()
        }
    }

    Ok(())
}

fn boot() {
    use clap::{ Arg, App };

    let matches = App::new("NASL Interpreter")
        .version("5.06")
        .author("Luozijun <luozijun.assistant@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("include_dir")
               .short("i")
               .long("include_dir")
               .help("Sets a custom config file")
               .required(true)
               .takes_value(true)
        )
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .possible_values(&["exec", "parse", "descr"])
                .required(true)
                .help("Sets a custom config file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("FILENAME")
                .help("Sets the input file to use")
                .required(true)
        )
        .arg(
            Arg::with_name("TARGET")
                .help("Sets the input file to use")
                .required(true)
        )
        .get_matches();


    let include_dir = matches.value_of("include_dir").unwrap_or("./");
    let mode = {
        let m = matches.value_of("mode").unwrap();
        if m == "exec" {
            0 | nasl_sys::NASL_COMMAND_LINE
        } else if m == "parse" {
            0 | nasl_sys::NASL_EXEC_PARSE_ONLY
        } else if m == "descr" {
            0 | nasl_sys::NASL_EXEC_DESCR
        } else {
            unreachable!();
        }
    };
    let filename = matches.value_of("FILENAME").unwrap();
    let target = matches.value_of("TARGET").unwrap();

    unsafe {
        let include_dir = fs::canonicalize(include_dir).unwrap().display().to_string();
        
        assert_eq!(nasl_sys::add_nasl_inc_dir(c_str(include_dir)), 0);
        assert_eq!(nasl_sys::add_nasl_inc_dir(c_str("./")), 0);
        assert_eq!(nasl_sys::add_nasl_inc_dir(c_str("./nasl_scripts")), 0);
        
        let _ = run_nasl(filename, mode, target);
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");

    env_logger::init();

    info!("nasl version: {:?}", version());
    
    boot();
}
