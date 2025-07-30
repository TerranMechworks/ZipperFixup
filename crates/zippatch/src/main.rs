mod hash;
mod patch;

use hash::HashCheck;
use patch::PatchError;

const KNOWN_EXES: &[&str] = &["Mech3", "Recoil", "Recoil3dfx", "RecoilD3D"];

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// https://github.com/rust-lang/rust/issues/79609
///
/// needed in the patcher, because overrides cannot be specified per-package
/// (yet)
#[cfg(target_env = "gnu")]
#[unsafe(no_mangle)]
pub extern "C" fn _Unwind_Resume() {}

fn main() {
    println!("=== ZipPatch {VERSION} ===");
    println!();

    let mut did_patch = 0;

    for base_name in KNOWN_EXES {
        let org_name = format!("{base_name}.exe");
        let new_name = format!("{base_name}fixup.exe");

        // TODO: should we hash the exe or check the size here to detect
        // whether the DLL will work?
        match std::fs::read(&org_name) {
            Ok(contents) => {
                println!("* '{org_name}' found, checking and patching...");
                match hash::hash_binary(&contents) {
                    HashCheck::Known(known) => {
                        println!("INFO: Identified executable as {known}");
                    }
                    HashCheck::Unknown(hash) => {
                        println!(
                            "WARNING: Unknown executable, ZipperFixup may not work as expected."
                        );
                        println!("         This can happen for several reasons:");
                        println!("         * You haven't installed the v1.2 patch");
                        println!("         * You have a version or language we don't know about");
                        println!("         * You have installed another patch (e.g. NoCD)");
                        println!("Developer information:");
                        println!("{hash}  {org_name}");
                    }
                }

                match patch::patch_binary(contents) {
                    Ok(contents) => match std::fs::write(&new_name, contents) {
                        Ok(()) => {
                            did_patch += 1;
                            println!("Patch OK - see '{new_name}'")
                        }
                        Err(e) => println!("Patch FAILED - error writing '{new_name}': {e}"),
                    },
                    Err(PatchError::AlreadyApplied) => {
                        println!("Patch FAILED - patch already applied to '{org_name}'")
                    }
                    Err(PatchError::NoCandidates) => println!("Patch FAILED - no candidates"),
                    Err(PatchError::MultipleCandidates) => {
                        println!("Patch FAILED - multiple candidates")
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("* '{org_name}' not found")
            }
            Err(e) => println!("Patch FAILED - error reading '{org_name}': {e}"),
        };
    }

    println!();
    println!("{did_patch} executables patched");
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
