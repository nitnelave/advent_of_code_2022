fn main() {
    println!("cargo:rustc-link-arg=-nostartfiles");
    //println!("cargo:rustc-target-cpu=native");
    println!(
        "cargo:rustc-link-arg=-Wl,-n,-N,--no-dynamic-linker,--build-id=none,--no-eh-frame-hdr"
    );
}
