use std::path::PathBuf;

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={root}/vendor/dynomath/lib");
    println!("cargo:rustc-link-lib=dynomath");
    println!("cargo:rerun-if-changed={root}/vendor/dynomath/include/dynomath.h");

    let bindings = bindgen::Builder::default()
        .header("vendor/dynomath/include/dynomath.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("dynomath.rs"))
        .expect("Couldn't write bindings!");
}
