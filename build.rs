//extern crate bindgen;
extern crate gcc;

// use std::env;

fn main() {
    /*
    let ndk_path = env::var("NDK_HOME").expect("Please set the path to the Android NDK with the \
                                                $NDK_HOME environment variable.");

    // let h_path = format!("{}/sources/android/cpufeatures/cpu-features.h", ndk_path);
    let c_path = format!("{}/sources/android/cpufeatures/cpu-features.c", ndk_path);
    */
    let c_path = "cpu-features-wrapper.c";

    gcc::compile_library("libcpufeatures.a", &[&c_path]);
    println!("cargo:rustc-link-lib=static=cpufeatures");

    /*
    let bindings = bindgen::Builder::default()
        .header(h_path)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    */
}
