#[macro_export]
macro_rules! dbg {
    (@encode $msg:ident) => {{
        let mut msg: Vec<u16> = $msg.encode_utf16().collect();
        msg.push(0);
        unsafe { ::winapi::um::debugapi::OutputDebugStringW(msg.as_ptr()) };
    }};
    ($fmt:literal) => {{
        let msg: &str = $fmt;
        $crate::dbg!(@encode msg);
    }};
    ($fmt:literal, $($args:expr),+ $(,)?) => {{
        let msg = format!($fmt, $($args,)+);
        $crate::dbg!(@encode msg);
    }};
}
