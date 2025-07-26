fn main() {
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();

    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = std::path::PathBuf::from(path);
    path.push("exports.def");
    let path = format!("{}", path.display());

    println!("cargo::warning={path}");
    println!("cargo::rerun-if-changed={path}");

    match env.as_str() {
        "gnu" => {
            println!("cargo::warning=GNU");
            println!("cargo::rustc-link-arg-cdylib={path}");
        }
        "msvc" => {
            println!("cargo::warning=MSVC");
            println!("cargo::rustc-link-arg-cdylib=/DEF:{path}");
        }
        _ => {
            println!("cargo::warning=unknown env `{env}`");
        }
    }
}
