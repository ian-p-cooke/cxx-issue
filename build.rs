use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-lib=static=pcap");
    println!("cargo:rustc-link-lib=static=Common++");
    println!("cargo:rustc-link-lib=static=Packet++");
    println!("cargo:rustc-link-lib=static=Pcap++");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let bindings = bindgen::Builder::default()
        .clang_arg("-std=c++17")
        .clang_arg("-I./include")
        .allowlist_type("pcpp::RawPacket")
        .allowlist_type("pcpp::TcpReassembly")
        .allowlist_type("pcpp::IFileReaderDevice")
        .opaque_type("pcpp::PointerVector")
        .opaque_type("std::.+")
        .enable_cxx_namespaces()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cxx_build::bridge("src/main.rs")
        .file("src/pcap_reader.cpp")
        .flag("-std=c++17")
        //.include("/home/icooke/opt/PcapPlusPlus/include")
        .compile("demo");
}
