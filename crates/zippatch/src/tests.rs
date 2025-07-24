use super::{PatchError, patch_binary};

#[test]
fn kernel32_ok() {
    let contents = b"***KERNEL32.dll***".to_vec();
    let res = patch_binary(contents);
    let expected = b"***ZIPFIXUP.dll***".to_vec();
    assert_eq!(res, Ok(expected));
}

#[test]
fn mech3fix_ok() {
    let contents = b"***MECH3FIX.dll***".to_vec();
    let res = patch_binary(contents);
    let expected = b"***ZIPFIXUP.dll***".to_vec();
    assert_eq!(res, Ok(expected));
}

#[test]
fn nothing_found() {
    let contents = b"******".to_vec();
    let res = patch_binary(contents);
    assert_eq!(res, Err(PatchError::NoCandidates));
}

#[test]
fn kernel32_dupe() {
    let contents = b"***KERNEL32.dll***KERNEL32.dll***".to_vec();
    let res = patch_binary(contents);
    assert_eq!(res, Err(PatchError::MultipleCandidates));
}

#[test]
fn mech3fix_dupe() {
    let contents = b"***MECH3FIX.dll***MECH3FIX.dll***".to_vec();
    let res = patch_binary(contents);
    assert_eq!(res, Err(PatchError::MultipleCandidates));
}

#[test]
fn dupes() {
    let contents = b"***KERNEL32.dll***MECH3FIX.dll***".to_vec();
    let res = patch_binary(contents);
    assert_eq!(res, Err(PatchError::MultipleCandidates));

    let contents = b"***MECH3FIX.dll***KERNEL32.dll***".to_vec();
    let res = patch_binary(contents);
    assert_eq!(res, Err(PatchError::MultipleCandidates));
}

#[test]
fn already_patched() {
    let contents = b"***ZIPFIXUP.dll***".to_vec();
    let res = patch_binary(contents);
    assert_eq!(res, Err(PatchError::AlreadyApplied));
}
