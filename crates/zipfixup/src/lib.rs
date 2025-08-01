#![cfg(windows)]
mod dbg;
mod dll_main;
mod err_msg;
mod hook;
mod mech3;
mod overrides;
mod patch;
mod recoil;

pub(crate) use dbg::output;
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// https://github.com/rust-lang/rust/issues/79609
#[cfg(target_env = "gnu")]
#[unsafe(no_mangle)]
pub extern "C" fn _Unwind_Resume() {}
