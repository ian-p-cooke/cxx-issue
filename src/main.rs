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

pub struct UserCookie {
    num_callbacks: u64,
}

unsafe fn onMessageReadyCallback(side: i8, tcpData: &crate::root::pcpp::TcpStreamData, userCookie: *mut UserCookie) {
    let cookie = &mut *userCookie;
    cookie.num_callbacks += 1;
}

#[cxx::bridge]
mod ffi {
    #[namespace = "pcpp"]
    extern "C++" {
        type TcpStreamData = crate::root::pcpp::TcpStreamData;
        type ConnectionData = crate::root::pcpp::ConnectionData;
    }

    extern "Rust" {
        type UserCookie;
        unsafe fn onMessageReadyCallback(side: i8, tcpData: &TcpStreamData, userCookie: *mut UserCookie);
    }

    unsafe extern "C++" {
        include!("demo/include/pcap_reader.hpp");
        fn read_pcap(fileName: &str,
                            userCookie: &mut UserCookie,
                            bpfFilter: &str);
    }
}

fn main() {
    let mut cookie = UserCookie { num_callbacks: 0 };
    ffi::read_pcap("my.pcap", &mut cookie, "");
    println!("num_callbacks: {}", cookie.num_callbacks);
}
