use object::read::pe::{Export, ExportTarget, PeFile32};
use std::collections::HashSet;
use std::fmt;

const DEF_FILE: &str = include_str!("../../zipfixup/exports.def");

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (expected_exports, expected_forwards) = parse_def_file()?;

    for path in std::env::args().skip(1) {
        println!("=== {} ===", path);

        let contents = std::fs::read(path)?;
        let data = contents.as_slice();
        let pe = PeFile32::parse(data)?;
        let table = pe.export_table()?.ok_or("no export table")?;
        let exported = table.exports()?;

        validate_exports(&exported, &expected_exports, &expected_forwards)?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Forward<'a> {
    export: &'a str,
    module: &'a str,
    forward: &'a str,
}

impl fmt::Display for Forward<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}.{}", self.export, self.module, self.forward)
    }
}

fn parse_def_file() -> Result<(Vec<&'static str>, Vec<Forward<'static>>), String> {
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

        let parse: Vec<&str> = line.split(&['=', '.']).collect();
        match &parse[..] {
            [export, module, forward] => {
                forwards.push(Forward {
                    export,
                    module,
                    forward,
                });
            }
            _ => {
                exports.push(line);
            }
        }
    }

    Ok((exports, forwards))
}

fn validate_exports(
    exported: &[Export<'_>],
    expected_exports: &[&'static str],
    expected_forwards: &[Forward<'static>],
) -> Result<(), String> {
    let mut exports = Vec::new();
    let mut forwards = Vec::new();
    for export in exported {
        let name = export
            .name
            .ok_or_else(|| format!("export has no name: {export:?}"))?;
        let name =
            str::from_utf8(name).map_err(|_e| format!("export name is not unicode: {export:?}"))?;

        match export.target {
            ExportTarget::ForwardByName(module, forward) => {
                let module = str::from_utf8(module)
                    .map_err(|_e| format!("export module is not unicode: {export:?}"))?;
                let forward = str::from_utf8(forward)
                    .map_err(|_e| format!("export forward is not unicode: {export:?}"))?;

                forwards.push(Forward {
                    export: name,
                    module,
                    forward,
                });
            }
            ExportTarget::Address(_addr) => {
                exports.push(name);
            }
            ExportTarget::ForwardByOrdinal(_module, _ordinal) => {
                return Err(format!("unexpected forward by ordinal: {export:?}"));
            }
        }
    }

    let mut actual_exports: HashSet<&str> = exports.iter().copied().collect();
    let mut actual_forwards: HashSet<Forward<'_>> = forwards.iter().copied().collect();

    exports.sort();
    forwards.sort();

    for name in exports.iter().copied() {
        println!("{name}");
    }

    for forward in forwards.iter().copied() {
        println!("{forward}");
    }

    for expected in expected_exports {
        if !actual_exports.remove(expected) {
            return Err(format!("missing export `{expected}`"));
        }
    }

    for expected in expected_forwards {
        if !actual_forwards.remove(expected) {
            return Err(format!("missing forward `{expected}`"));
        }
    }

    if !actual_forwards.is_empty() {
        return Err(format!("unexpected forwards: {actual_forwards:?}"));
    }

    Ok(())
}
