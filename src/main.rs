const DLL_NAME_LEN: usize = 12;
const KERNEL32_DLL: &[u8; DLL_NAME_LEN] = b"KERNEL32.dll";
/// Old project name
const MECH3FIX_DLL: &[u8; DLL_NAME_LEN] = b"MECH3FIX.dll";
const ZIPFIXUP_DLL: &[u8; DLL_NAME_LEN] = b"ZIPFIXUP.dll";

fn patch_binary(exe_data: &mut [u8]) -> bool {
    let mut found: Vec<usize> = Vec::new();
    // find `KERNEL32.dll`
    let mut it = exe_data.windows(DLL_NAME_LEN);
    while let Some(pos) = it.position(|window| window == KERNEL32_DLL) {
        found.push(pos);
    }
    drop(it);
    // find `MECH3FIX.dll`
    let mut it = exe_data.windows(DLL_NAME_LEN);
    while let Some(pos) = it.position(|window| window == MECH3FIX_DLL) {
        found.push(pos);
    }
    drop(it);

    match found[..] {
        [start] => {
            // There should only be one instance
            let end = start + DLL_NAME_LEN;
            exe_data[start..end].copy_from_slice(ZIPFIXUP_DLL);
            println!("Patch applied, writing file...");
            true
        }
        [] => {
            println!("Patch FAILED - maybe already applied?");
            false
        }
        _ => {
            println!("Patch FAILED - multiple candidates");
            false
        }
    }
}

fn main() {
    println!();
    println!("Finding exe to patch...");

    for org_name in &["Mech3.exe", "Recoil.exe", "Recoil3dfx.exe", "RecoilD3D.exe"] {
        let exe_base = org_name.strip_suffix(".exe").unwrap_or(org_name);
        let new_name = format!("{}fixup.exe", exe_base);

        match std::fs::read(org_name) {
            Ok(mut exe_data) => {
                println!("'{}' found, patching...", org_name);
                if patch_binary(&mut exe_data) {
                    match std::fs::write(&new_name, exe_data) {
                        Ok(()) => println!("Patch SUCCESS - see '{}'", new_name),
                        Err(e) => println!("Error writing '{}', continuing... ({:?})", new_name, e),
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("'{}' not found", org_name)
            }
            Err(e) => println!("Error reading '{}', continuing... ({:?})", org_name, e),
        };
    }
}
