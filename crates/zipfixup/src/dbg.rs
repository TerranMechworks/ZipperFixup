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
    // OutputDebugStringW is... weird/standard Microsoft:
    // > `OutputDebugStringW` converts the specified string based on the current
    // > system locale information and passes it to `OutputDebugStringA` to be
    // > displayed. As a result, some Unicode characters may not be displayed
    // > correctly.
    // https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringw
    // Therefore, we may be better off just using OutputDebugStringA...
    unsafe { OutputDebugStringW(p) };
    // paranoia: ensure `v` is valid until after `OutputDebugStringW`
    drop(v);
}

pub(crate) fn encode_ascii(msg: &str) {
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

    let v: Vec<i8> = s
        .chars()
        .map(|c| {
            let b = if c.is_ascii() { c as u8 } else { b'?' };
            b as i8
        })
        .collect();
    let p: *const i8 = v.as_ptr();
    unsafe { OutputDebugStringA(p) };
    // paranoia: ensure `s` is valid until after `OutputDebugStringA`
    drop(s);
}

macro_rules! output {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        let msg: String = format!($fmt $(, $args)*);
        $crate::dbg::encode_ascii(&msg);
    }};
    (u $fmt:literal $(, $args:expr)* $(,)?) => {{
        let msg: String = format!($fmt $(, $args)*);
        $crate::dbg::encode_unicode(&msg);
    }};
}
pub(crate) use output;
