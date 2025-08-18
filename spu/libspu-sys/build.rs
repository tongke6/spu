fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/bridge.cc")
        .file("src/link.cc")
        .include("src")
        .include("external/yacl") // Add yacl include path
        .flag_if_supported("-std=c++17")
        .compile("libspu-sys");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/bridge.cc");
    println!("cargo:rerun-if-changed=src/link.cc");
}
