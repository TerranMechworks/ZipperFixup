use super::PatchError;
use region::Protection;

/// Patch a single byte.
///
/// **WARNING**: This is racy and must be run in a critical section.
#[expect(dead_code)]
pub(crate) fn byte(addr: u32, value: u8, expected: u8) -> Result<(), PatchError> {
    let byte_addr = addr as *mut u8;

    // Check existing assembly (sanity)
    let actual = unsafe { byte_addr.read_unaligned() };

    if actual != expected {
        return Err(PatchError::Byte {
            offset: addr,
            actual,
            expected,
        });
    }

    // Unprotect
    unsafe { region::protect(byte_addr, 1, Protection::READ_WRITE_EXECUTE) }?;
    // Write new value
    unsafe { byte_addr.write_unaligned(value) };

    Ok(())
}
