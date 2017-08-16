extern crate cpp_build;
extern crate pkg_config;

fn main() {
    cpp_build::Config::new()
        .include("/usr/local/include/JSBSim")
        .build("src/main.rs");
    println!("cargo:rustc-link-lib=JSBSim");
}
