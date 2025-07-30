const DLL_NAME_LEN: usize = 12;
const KERNEL32_DLL: &[u8; DLL_NAME_LEN] = b"KERNEL32.dll";
/// Old project name
const MECH3FIX_DLL: &[u8; DLL_NAME_LEN] = b"MECH3FIX.dll";
const ZIPFIXUP_DLL: &[u8; DLL_NAME_LEN] = b"ZIPFIXUP.dll";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PatchError {
    AlreadyApplied,
    NoCandidates,
    MultipleCandidates,
}

pub(crate) fn patch_binary(mut contents: Vec<u8>) -> Result<Vec<u8>, PatchError> {
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
