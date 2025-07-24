const DLL_NAME_LEN: usize = 12;
const KERNEL32_DLL: &[u8; DLL_NAME_LEN] = b"KERNEL32.dll";
/// Old project name
const MECH3FIX_DLL: &[u8; DLL_NAME_LEN] = b"MECH3FIX.dll";
const ZIPFIXUP_DLL: &[u8; DLL_NAME_LEN] = b"ZIPFIXUP.dll";

const KNOWN_EXES: &[&str] = &["Mech3", "Recoil", "Recoil3dfx", "RecoilD3D"];

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// https://github.com/rust-lang/rust/issues/79609
///
/// needed in the patcher, because overrides cannot be specified per-package
/// (yet)
#[unsafe(no_mangle)]
pub extern "C" fn _Unwind_Resume() {}

fn main() {
    println!("=== ZipPatch {} ===", VERSION);
    println!();

    let mut did_patch = 0;

    for base_name in KNOWN_EXES {
        let org_name = format!("{}.exe", base_name);
        let new_name = format!("{}fixup.exe", base_name);

        match std::fs::read(&org_name) {
            Ok(contents) => {
                println!("* '{}' found, patching...", org_name);
                match patch_binary(contents) {
                    Ok(contents) => match std::fs::write(&new_name, contents) {
                        Ok(()) => {
                            did_patch += 1;
                            println!("Patch OK - see '{}'", new_name)
                        }
                        Err(e) => println!("Patch FAILED - error writing '{}': {}", new_name, e),
                    },
                    Err(PatchError::AlreadyApplied) => {
                        println!("Patch FAILED - patch already applied to '{}'", org_name)
                    }
                    Err(PatchError::NoCandidates) => println!("Patch FAILED - no candidates"),
                    Err(PatchError::MultipleCandidates) => {
                        println!("Patch FAILED - multiple candidates")
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("* '{}' not found", org_name)
            }
            Err(e) => println!("Patch FAILED - error reading '{}': {}", org_name, e),
        };
    }

    println!();
    println!("{} executables patched", did_patch);
    println!();

    wait_for_enter();
}

fn wait_for_enter() {
    print!("Press enter to exit");
    use std::io::Write as _;
    let _ = std::io::stdout().flush();
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatchError {
    AlreadyApplied,
    NoCandidates,
    MultipleCandidates,
}

fn patch_binary(mut contents: Vec<u8>) -> Result<Vec<u8>, PatchError> {
    let mut found: Vec<usize> = Vec::new();

    // find `KERNEL32.dll`
    let mut it = contents.windows(DLL_NAME_LEN);
    while let Some(pos) = it.position(|window| window == KERNEL32_DLL) {
        found.push(pos);
    }
    drop(it);

    // find `MECH3FIX.dll`
    let mut it = contents.windows(DLL_NAME_LEN);
    while let Some(pos) = it.position(|window| window == MECH3FIX_DLL) {
        found.push(pos);
    }
    drop(it);

    // find `ZIPFIXUP.dll`
    let mut it = contents.windows(DLL_NAME_LEN);
    let already_patched = it.any(|window| window == ZIPFIXUP_DLL);
    drop(it);

    match found[..] {
        [start] => {
            let end = start + DLL_NAME_LEN;
            contents[start..end].copy_from_slice(ZIPFIXUP_DLL);
            Ok(contents)
        }
        [] => match already_patched {
            true => Err(PatchError::AlreadyApplied),
            false => Err(PatchError::NoCandidates),
        },
        _ => Err(PatchError::MultipleCandidates),
    }
}

#[cfg(test)]
mod tests;
