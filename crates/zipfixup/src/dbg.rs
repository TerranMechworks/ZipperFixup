use ::winapi::um::debugapi::{OutputDebugStringA, OutputDebugStringW};

#[expect(dead_code, reason = "test me!")]
pub(crate) fn encode_unicode(msg: &str) {
    let now = time::OffsetDateTime::now_utc();
    let s = format!(
        "[ZF {:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z] {msg}\0",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.millisecond(),
    );

    let v: Vec<u16> = s.encode_utf16().collect();
    let p: *const u16 = v.as_ptr();
    unsafe { OutputDebugStringW(p) };
    // paranoia: ensure `v` is valid until after `OutputDebugStringW`
    drop(v);
}

/// WARNING: message must be ASCII!
pub(crate) unsafe fn encode_ascii(msg: &str) {
    let now = time::OffsetDateTime::now_utc();
    let s = format!(
        "[ZF {:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z] {msg}\0",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.millisecond(),
    );

    let p: *const i8 = s.as_ptr() as *const i8;
    unsafe { OutputDebugStringA(p) };
    // paranoia: ensure `s` is valid until after `OutputDebugStringW`
    drop(s);
}

/// WARNING: message must be ASCII!
///
/// In theory, we could improve this by using [`OutputDebugStringW`] and
/// [`str::encode_utf16`], see [`encode_unicode`]. That can also automatically
/// add zero-termination, so `"\0"` is no longer necessary.
///
/// I think I tested that though, and `OutputDebugStringW` didn't work?
macro_rules! output {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        let msg: String = format!($fmt $(, $args)*);
        unsafe { $crate::dbg::encode_ascii(&msg) };
    }};
    (u $fmt:literal $(, $args:expr)* $(,)?) => {{
        let msg: String = format!($fmt $(, $args)*);
        $crate::dbg::encode_unicode(&msg);
    }};
}
pub(crate) use output;
