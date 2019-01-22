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
use std::net::{ Ipv4Addr, IpAddr, };



pub fn c_str<T: Into<Vec<u8>>>(t: T) -> *mut libc::c_char {
    CString::new(t).unwrap().into_raw()
}

pub struct Vm {

}

impl Vm {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            nasl_sys::gcrypt_init();
            nasl_sys::openvas_SSL_init();
        }

        Ok( Vm {} )
    }

    pub fn version(&self) -> String {
        unsafe {
            CStr::from_ptr(nasl_sys::nasl_version())
                .to_str().unwrap().to_string()
        }
    }

    pub fn add_include_dir(&self, dirname: &str) -> Result<(), ()> {
        let include_dir = fs::canonicalize(dirname).unwrap().display().to_string();
        unsafe {
            if nasl_sys::add_nasl_inc_dir(c_str(include_dir)) != 0 {
                return Err(())
            } else {
                return Ok(())
            }
        }
    }

    pub fn parse(&self, _target: &IpAddr, _script_name: &str) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn description(&self, script_name: &str) -> Result<(), ()> {
        unsafe {
            let mut ip6: libc::in6_addr = mem::zeroed();
            ip6.s6_addr = Ipv4Addr::new(127, 0, 0, 1).to_ipv6_mapped().octets();

            let vclass = *nasl_sys::KBDefaultOperations;

            assert_eq!(vclass.kb_new.is_some(), true);
            let kb_new = vclass.kb_new.unwrap();

            let mut kb: nasl_sys::kb_t = ptr::null_mut();

            if kb_new(&mut kb, c_str(nasl_sys::KB_PATH_DEFAULT)) > 0 {
                error!("kb_new failed.");
                return Err(());
            }

            let mut script_infos = nasl_sys::init(&mut ip6, ptr::null_mut(), kb);
            
            (*script_infos).name = c_str(script_name);

            let nvti: *mut nasl_sys::nvti_t = nasl_sys::parse_script_infos(script_infos);
            if nvti.is_null() {
                error!("parse_script_infos failed.");
                return Err(());
            }
        }

        Ok(())
    }

    pub fn attack(&self, target: &IpAddr, script_name: &str) -> Result<(), ()> {
        unsafe {
            let mut ip6: libc::in6_addr = mem::zeroed();
            let v6_octets = match target {
                IpAddr::V4(v4_addr) => v4_addr.to_ipv6_mapped().octets(),
                IpAddr::V6(v6_addr) => v6_addr.octets(),
            };
            ip6.s6_addr = v6_octets;


            let vclass = *nasl_sys::KBDefaultOperations;

            assert_eq!(vclass.kb_new.is_some(), true);
            let kb_new = vclass.kb_new.unwrap();

            let mut kb: nasl_sys::kb_t = ptr::null_mut();

            if kb_new(&mut kb, c_str(nasl_sys::KB_PATH_DEFAULT)) > 0 {
                error!("kb_new failed.");
                return Err(());
            }

            let mut script_infos = nasl_sys::init(&mut ip6, ptr::null_mut(), kb);
            
            (*script_infos).name = c_str(script_name);

            let ret_code = nasl_sys::exec_nasl_script(script_infos, nasl_sys::NASL_COMMAND_LINE);
            if ret_code != 0 {
                error!("exec_nasl_script ret code: {:?}", ret_code);
                return Err(());
            }
        }
        
        Ok(())
    }
}


pub struct Host {
    inner: *mut nasl_sys::gvm_host_t
}

impl Host {
    pub fn get_in6_addr(&self) -> libc::in6_addr {
        unsafe {
            let mut ip6: libc::in6_addr = mem::zeroed();
            nasl_sys::gvm_host_get_addr6(self.inner, &mut ip6);
            ip6
        }
    }
}

pub unsafe fn get_hosts(target: &str) -> Vec<Host> {
    let hosts = nasl_sys::gvm_hosts_new(c_str(target));
    nasl_sys::gvm_hosts_resolve(hosts);

    let mut hosts2 = Vec::new();
    loop {
        let host = nasl_sys::gvm_hosts_next (hosts);
        
        if host.is_null() {
            break;
        }

        hosts2.push( Host { inner: host as _ });
    }

    hosts2
}


// set_kb_item(name:"Settings/disable_cgi_scanning", value:FALSE);
// set_kb_item(name:"Services/www", value:80);
// set_kb_item(name:"Host/scanned", value:TRUE);
// set_kb_item(name:"Ports/tcp/80", value:TRUE);
// set_kb_item(name:"Transports/TCP/80", value:TRUE);

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
    let target = {
        let target_str = matches.value_of("TARGET").unwrap();
        match target_str.parse::<IpAddr>() {
            Ok(ip_addr) => ip_addr,
            Err(e) => {
                error!("{:?}", e);
                return ();
            }
        }
    };

    let vm = Vm::new().unwrap();
    vm.add_include_dir(include_dir).unwrap();
    vm.add_include_dir("./").unwrap();
    vm.add_include_dir("./nasl_scripts").unwrap();

    info!("nasl version: {:?}", vm.version());

    info!("Exec description ...");
    vm.description(filename).unwrap();
    info!("attack ...");
    vm.attack(&target, filename).unwrap();
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    boot();
}
