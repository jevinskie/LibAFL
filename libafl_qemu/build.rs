mod host_specific {
    #[cfg(target_os = "linux")]
    include!("build_linux.rs");

    #[cfg(target_os = "macos")]
    include!("build_linux.rs");

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    pub fn build() {
        println!("cargo:warning=libafl_qemu only builds on Linux hosts");
    }
}

#[rustversion::nightly]
fn nightly() {
    println!("cargo:rustc-cfg=nightly");
}

#[rustversion::not(nightly)]
fn nightly() {}

fn main() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
    nightly();
    host_specific::build();
}
