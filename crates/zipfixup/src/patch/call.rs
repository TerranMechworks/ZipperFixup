use super::PatchError;
use region::Protection;

fn rel_offset(call_site: u32, func_addr: *const ()) -> u32 {
    let addr = func_addr as u32;
    let eip = call_site.wrapping_add(5);
    addr.wrapping_sub(eip)
}

/// Patch a `CALL <rel>` instruction.
///
/// **WARNING**: This is racy and must be run in a critical section.
pub(crate) fn call(
    call_site: u32,
    func_addr: *const (),
    expected_rel: u32,
) -> Result<(), PatchError> {
    // Calculate new relative offset
    let rel = rel_offset(call_site, func_addr);

    let call_base = call_site as *mut u8;
    // Relative offset after CALL/e8 <rel>
    let call_rel = call_site.wrapping_add(1);
    let call_addr = call_rel as *mut u32;

    // Check existing assembly (sanity)
    let actual = unsafe { call_base.read_unaligned() };

    if actual != 0xe8 {
        return Err(PatchError::Byte {
            offset: call_site,
            actual,
            expected: 0xe8,
        });
    }

    // Check existing relative offset (sanity)
    let actual = unsafe { call_addr.read_unaligned() };

    if actual != expected_rel {
        return Err(PatchError::Dword {
            offset: call_rel,
            actual,
            expected: expected_rel,
        });
    }

    // Unprotect
    unsafe { region::protect(call_addr, 4, Protection::READ_WRITE_EXECUTE) }?;
    // Write new relative offset (very likely to be unaligned...)
    unsafe { call_addr.write_unaligned(rel) };

    Ok(())
}
