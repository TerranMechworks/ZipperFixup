use crate::{Result, output};
use retour::RawDetour;
use std::ffi::CStr;
use std::sync::OnceLock;

unsafe extern "C" {
    fn err_msg();
}

#[unsafe(no_mangle)]
extern "C" fn recoil_log_formatted(flags: i32, path: *const i8, line: i32, buffer: *const i8) {
    let path = unsafe { CStr::from_ptr(path) };
    let buffer = unsafe { CStr::from_ptr(buffer) };

    let path = path.to_string_lossy();
    let buffer = buffer.to_string_lossy();

    output!("{path}:{line} ({flags:#08X})\n{buffer}");
}

static ERR_MSG: OnceLock<RawDetour> = OnceLock::new();

const ERR_MSG_ADDR: *const () = 0x00404e80 as *const ();
const ERR_MSG_DETOUR: *const () = err_msg as *const ();

pub(crate) fn install_hooks() -> Result<()> {
    output!("Installing hooks... (RC)");

    output!("Hooking ErrMsg...");

    let detour = unsafe { RawDetour::new(ERR_MSG_ADDR, ERR_MSG_DETOUR) }
        .map_err(|e| format!("failed to detour ErrMsg: {e}"))?;

    unsafe { detour.enable() }.map_err(|e| format!("failed to enable ErrMsg: {e}"))?;

    ERR_MSG
        .set(detour)
        .map_err(|_| format!("failed to set ErrMsg hook: already set"))?;

    output!("Hooked ErrMsg");

    output!("Installed hooks");
    Ok(())
}
