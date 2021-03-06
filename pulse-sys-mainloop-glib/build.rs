#[cfg(target_os="linux")]
extern crate pkg_config;

#[cfg(target_os="linux")]
fn main() {
    let min_version = "10.0";
    // Try package-config first
    let pc = pkg_config::Config::new().atleast_version(min_version).probe("libpulse-mainloop-glib");
    // Fallback to hard-coded on error (useful if user does not have *.pc file installed)
    if pc.is_err() {
        println!("cargo:rustc-link-lib=pulse-mainloop-glib::libpulse-mainloop-glib.so.0");
    }
}

#[cfg(not(target_os="linux"))]
fn main() {
    println!("cargo:rustc-link-lib=pulse-mainloop-glib");
}
