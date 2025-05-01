fn main() {
    let _ = cxx_build::bridge("src/ffi.rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
