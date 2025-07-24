use ::winapi::um::debugapi::{OutputDebugStringA, OutputDebugStringW};

/// Adds zero-termination.
#[expect(dead_code, reason = "test me!")]
pub(crate) unsafe fn encode_unicode(s: &str) {
    let v: Vec<u16> = s.encode_utf16().chain(Some(0)).collect();
    let p: *const u16 = v.as_ptr();
    unsafe { OutputDebugStringW(p) };
    // paranoia: ensure `v` is valid until after `OutputDebugStringW`
    drop(v);
}

/// WARNING: message must be ASCII!
pub(crate) unsafe fn encode_ascii(s: &str) {
    let p: *const i8 = s.as_ptr() as *const i8;
    unsafe { OutputDebugStringA(p) };
}

/// WARNING: message must be ASCII!
///
/// In theory, we could improve this by using [`OutputDebugStringW`] and
/// [`str::encode_utf16`], see [`encode_unicode`]. That can also automatically
/// add zero-termination, so `"\0"` is no longer necessary.
///
/// I think I tested that though, and `OutputDebugStringW` didn't work?
macro_rules! output {
    ($fmt:literal) => {{
        let msg: &str = concat!("[ZIPFIXUP] ", $fmt, "\0");
        unsafe { $crate::dbg::encode_ascii(msg) };
    }};
    ($fmt:literal, $($args:expr),+ $(,)?) => {{
        let msg: String = format!(concat!("[ZIPFIXUP] ", $fmt, "\0"), $($args,)+);
        unsafe { $crate::dbg::encode_ascii(&msg) };
    }};
}
pub(crate) use output;
