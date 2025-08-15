//! [`output!`] macro to log messages via the Debug API, to be viewed in e.g.
//! [DebugView](https://learn.microsoft.com/en-us/sysinternals/downloads/debugview).
use ::winapi::um::debugapi::{OutputDebugStringA, OutputDebugStringW};

macro_rules! output {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        let msg: String = format!($fmt $(, $args)*);
        $crate::dbg::output_debug_string_w(&msg);
    }};
    (a $fmt:literal $(, $args:expr)* $(,)?) => {{
        let msg: String = format!($fmt $(, $args)*);
        $crate::dbg::output_debug_string_a(&msg);
    }};
}
pub(crate) use output;

fn encode_unicode(msg: &str) -> Vec<u16> {
    const ZF: &[u16] = &[0x5b, 0x5a, 0x46, 0x5d, 0x20]; // "[ZF] "
    ZF.iter()
        .copied()
        .chain(msg.encode_utf16().chain(Some(0)))
        .collect()
}

/// Output a Unicode debug string.
///
/// OutputDebugStringW is... weird/standard Microsoft:
/// > `OutputDebugStringW` converts the specified string based on the current
/// > system locale information and passes it to `OutputDebugStringA` to be
/// > displayed. As a result, some Unicode characters may not be displayed
/// > correctly.
///
/// See <https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringw>.
///
/// Although you shouldn't log a lot of stuff, if you need to, the ASCII
/// version may be slightly faster.
pub(crate) fn output_debug_string_w(msg: &str) {
    let v: Vec<u16> = encode_unicode(msg);
    let p: *const u16 = v.as_ptr();
    unsafe { OutputDebugStringW(p) };
    // paranoia: ensure `v` is valid until after `OutputDebugStringW`
    drop(v);
}

fn encode_ascii(msg: &str) -> Vec<i8> {
    const ZF: &[u8] = b"[ZF] ";
    let msg = msg
        .chars()
        .map(|c| if c.is_ascii() { c as u8 } else { b'?' });
    ZF.iter()
        .copied()
        .chain(msg.chain(Some(0)))
        .map(|b| b as i8)
        .collect()
}

/// Output an ASCII debug string.
///
/// Non-ASCII characters are replaced by `?`. This version may be slightly
/// faster than the Unicode version, as it avoids extra translation (due to
/// Microsoft Unicode ineptness).
#[allow(dead_code, reason = "Use Unicode version by default")]
pub(crate) fn output_debug_string_a(msg: &str) {
    let v: Vec<i8> = encode_ascii(&msg);
    let p: *const i8 = v.as_ptr();
    unsafe { OutputDebugStringA(p) };
    // paranoia: ensure `v` is valid until after `OutputDebugStringA`
    drop(v);
}
