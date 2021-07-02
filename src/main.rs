#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use cxx::{type_id, ExternType};
unsafe impl ExternType for root::pcpp::TcpStreamData {
    type Id = type_id!("pcpp::TcpStreamData");
    type Kind = cxx::kind::Opaque;
}
unsafe impl ExternType for root::pcpp::ConnectionData {
    type Id = type_id!("pcpp::ConnectionData");
    type Kind = cxx::kind::Opaque;
}

#[cxx::bridge(namespace = "pcpp")]
mod ffi_ppcp {
    extern "C++" {
        type TcpStreamData = crate::root::pcpp::TcpStreamData;
        type ConnectionData = crate::root::pcpp::ConnectionData;
    }
}

struct UserCookie;

fn onMessageReadyCallback(side: i8, tcpData: &crate::root::pcpp::TcpStreamData, userCookie: *mut ::std::os::raw::c_void) {
    println!("onMessageReadyCallback!");
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type UserCookie;
        fn onMessageReadyCallback(side: i8, tcpData: &crate::ffi_pcpp::TcpStreamData, userCookie: *mut ::std::os::raw::c_void);
    }

    extern "C++" {
        include!("demo/include/pcap_reader.hpp");
        unsafe fn read_pcap(fileName: &str,
                            userCookie: &mut UserCookie,
                            bpfFilter: &str);
    }
}

fn main() {
}
