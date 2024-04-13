use std::env;
use std::fs;
use std::path::Path;


fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    watch_trndll64();


}


/// Watch the TRNDll64.lib file and rebuild if it changes
fn watch_trndll64() {
    println!("cargo::rerun-if-changed=lib/TRNDll64.lib");
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=TRNDll64");
}

