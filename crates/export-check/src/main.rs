use object::read::pe::{ExportTarget, PeFile32};
use std::collections::HashSet;

const DEF_FILE: &str = include_str!("../../zipfixup/exports.def");

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (expected_exports, expected_forwards) = parse_def_file()?;

    let contents = std::fs::read("target/i686-pc-windows-gnu/release/zipfixup.dll")?;
    let data = contents.as_slice();

    let pe = PeFile32::parse(data)?;
    let table = pe.export_table()?.ok_or("no export table")?;
    let exported = table.exports()?;

    let mut exports = Vec::new();
    let mut forwards = Vec::new();
    for export in &exported {
        let name = export
            .name
            .ok_or_else(|| format!("export has no name: {export:?}"))?;
        let name = str::from_utf8(name)?;

        match export.target {
            ExportTarget::ForwardByName(module, forward) => {
                if module != b"KERNEL32" {
                    return Err(format!("expected forward to KERNEL32: {export:?}").into());
                }
                if name.as_bytes() != forward {
                    return Err(format!(
                        "forward name mismatch `{name}` != `{}`",
                        forward.escape_ascii()
                    )
                    .into());
                }
                forwards.push(name);
            }
            ExportTarget::Address(_addr) => {
                exports.push(name);
            }
            ExportTarget::ForwardByOrdinal(module, _ordinal) => {
                if module != b"KERNEL32" {
                    return Err(format!("expected forward to KERNEL32: {export:?}").into());
                }
                return Err(format!("unexpected forward by ordinal: {export:?}").into());
            }
        }
    }

    let mut actual_exports: HashSet<&str> = exports.iter().copied().collect();
    let mut actual_forwards: HashSet<&str> = forwards.iter().copied().collect();

    exports.sort();
    forwards.sort();

    for name in exports.iter().copied() {
        println!("{name}");
    }

    for name in forwards.iter().copied() {
        println!("KERNEL32.{name}");
    }

    for expected in expected_exports {
        if !actual_exports.remove(&expected) {
            return Err(format!("missing export `{expected}`").into());
        }
    }

    for expected in expected_forwards {
        if !actual_forwards.remove(&expected) {
            return Err(format!("missing forward `{expected}`").into());
        }
    }

    if !actual_forwards.is_empty() {
        return Err(format!("unexpected forwards: {actual_forwards:?}").into());
    }

    Ok(())
}

fn parse_def_file() -> Result<(Vec<&'static str>, Vec<&'static str>), String> {
    let mut it = DEF_FILE.lines();
    let line = it.next().unwrap();
    if line != "LIBRARY ZIPFIXUP" {
        return Err(format!("expected `LIBRARY ZIPFIXUP`, found `{line}`"));
    }
    let line = it.next().unwrap();
    if line != "EXPORTS" {
        return Err(format!("expected `EXPORTS`, found `{line}`"));
    }

    let mut exports = Vec::new();
    let mut forwards = Vec::new();

    for line in it {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match line.split_once("=KERNEL32.") {
            Some((fwd, name)) => {
                if fwd != name {
                    return Err(format!("expected `{fwd}` == `{name}`"));
                }
                forwards.push(name);
            }
            None => {
                exports.push(line);
            }
        }
    }

    Ok((exports, forwards))
}
