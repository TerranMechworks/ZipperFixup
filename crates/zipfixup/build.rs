fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = std::path::PathBuf::from(path);
    path.push("exports.def");
    println!("cargo::warning={}", path.display());
    println!("cargo::rerun-if-changed={}", path.display());
    println!("cargo::rustc-link-arg-cdylib={}", path.display());
}
