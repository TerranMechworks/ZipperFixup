use crate::output;
use std::ffi::CStr;

unsafe extern "C" {
    fn err_msg_printf();
}

pub(crate) const ERR_MSG_ADDR: *const () = err_msg_printf as *const ();

#[unsafe(no_mangle)]
extern "C" fn err_msg_log(flags: i32, path: *const i8, line: i32, buffer: *const i8) {
    let path = unsafe { CStr::from_ptr(path) };
    let buffer = unsafe { CStr::from_ptr(buffer) };

    let path = path.to_string_lossy();
    let buffer = buffer.to_string_lossy();

    output!("{path}:{line} ({flags:#08X})\n{buffer}");
}
