#[macro_export]
macro_rules! output {
    (@encode $msg:ident) => {{
        unsafe { ::winapi::um::debugapi::OutputDebugStringA($msg.as_ptr() as *const i8) };
    }};
    ($fmt:literal) => {{
        let msg: &str = concat!("[ZIPFIXUP] ", $fmt, "\0");
        $crate::output!(@encode msg);
    }};
    ($fmt:literal, $($args:expr),+ $(,)?) => {{
        let msg: String = format!(concat!("[ZIPFIXUP] ", $fmt, "\0"), $($args,)+);
        $crate::output!(@encode msg);
    }};
}
