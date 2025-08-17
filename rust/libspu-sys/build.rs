fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/libspu/spu.cc") // This is not right, I'll fix it later.
        .include("src")
        .flag_if_supported("-std=c++17")
        .compile("libspu-sys");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
