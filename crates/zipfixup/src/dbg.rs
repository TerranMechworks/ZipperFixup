use ::winapi::um::debugapi::{OutputDebugStringA, OutputDebugStringW};

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

    let v: Vec<i8> = encode_ascii(&s);
    let p: *const i8 = v.as_ptr();
    unsafe { OutputDebugStringA(p) };
    // paranoia: ensure `s` is valid until after `OutputDebugStringA`
    drop(s);
}

macro_rules! output {
    (a $fmt:literal $(, $args:expr)* $(,)?) => {{
        let msg: String = format!($fmt $(, $args)*);
        $crate::dbg::output_debug_string_a(&msg);
    }};
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        let msg: String = format!($fmt $(, $args)*);
        $crate::dbg::output_debug_string_w(&msg);
    }};
}
pub(crate) use output;
