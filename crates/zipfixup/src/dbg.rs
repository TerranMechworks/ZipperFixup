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
    let s = format_log_msg(msg);
    let v: Vec<u16> = s.encode_utf16().collect();
    let p: *const u16 = v.as_ptr();
    unsafe { OutputDebugStringW(p) };
    // paranoia: ensure `v` is valid until after `OutputDebugStringW`
    drop(v);
}

fn encode_ascii(s: &str) -> Vec<i8> {
    s.chars()
        .map(|c| {
            let b = if c.is_ascii() { c as u8 } else { b'?' };
            b as i8
        })
        .collect()
}

/// Output an ASCII debug string.
///
/// Non-ASCII characters are replaced by `?`. This version may be slightly
/// faster than the Unicode version, as it avoids extra translation (due to
/// Microsoft Unicode ineptness).
#[allow(dead_code, reason = "Use Unicode version by default")]
pub(crate) fn output_debug_string_a(msg: &str) {
    let s = format_log_msg(msg);
    let v: Vec<i8> = encode_ascii(&s);
    let p: *const i8 = v.as_ptr();
    unsafe { OutputDebugStringA(p) };
    // paranoia: ensure `v` is valid until after `OutputDebugStringA`
    drop(v);
}

fn format_log_msg(msg: &str) -> String {
    let now = time::OffsetDateTime::now_utc();
    format!(
        "[ZF {:02}:{:02}:{:02}.{:03}Z] {msg}\0",
        now.hour(),
        now.minute(),
        now.second(),
        now.millisecond(),
    )
}
