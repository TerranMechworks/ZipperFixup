extern crate cc;

fn main() {
    cc::Build::new()
        .file("exports/exports.c")
        .compile("exports");
}
