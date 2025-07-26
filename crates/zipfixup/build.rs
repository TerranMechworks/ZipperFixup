fn main() {
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();
    println!("cargo::warning={env}");

    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = std::path::PathBuf::from(path);
    path.push("exports.def");
    let path = format!("{}", path.display());

    println!("cargo::warning={path}");
    println!("cargo::rerun-if-changed={path}");

    #[allow(clippy::wildcard_in_or_patterns)]
    match env.as_str() {
        "gnu" => {
            println!("cargo::rustc-link-arg-cdylib={path}");
        }
        "msvc" | _ => {
            println!("cargo::rustc-link-arg-cdylib=/DEF:{path}");
        }
    }
}
