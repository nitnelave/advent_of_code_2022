fn main() {
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-relocation-model=static");
    println!(
    "cargo:rustc-link-arg=-Wl,-n,-N,--static,--no-dynamic-linker,--build-id=none,--no-eh-frame-hdr"
);
}
