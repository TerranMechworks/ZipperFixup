//! [`DllMain`] logic, also installs specific hooks/patches based on the
//! executable that loaded the DLL.
use crate::{Result, output};
use std::sync::Mutex;
use windows::Win32::Foundation::{FALSE, GetLastError, HMODULE, TRUE};
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::System::Threading::{GetCurrentThread, QueueUserAPC};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(
    _hlibmodule: HMODULE,
    call_reason: u32,
    _reserved: *mut std::ffi::c_void,
) -> windows::core::BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            output!("DLL_PROCESS_ATTACH");
            let hthread = unsafe { GetCurrentThread() };
            let res = unsafe { QueueUserAPC(Some(load_fixup), hthread, 0) };
            if res == 0 {
                let e = unsafe { GetLastError() }.0;
                output!("QueueUserAPC failed: {e:08x}");
                FALSE
            } else {
                TRUE
            }
        }
        _ => TRUE,
    }
}

unsafe extern "system" fn load_fixup(_data: usize) {
    if let Err(e) = load_fixup_inner() {
        output!("FATAL error when loading fixup: {:?}", e);
    }
}

static INSTALLED: Mutex<bool> = Mutex::new(false);

fn load_fixup_inner() -> Result<()> {
    output!("Fixup loaded ({})", VERSION);

    let mut installed = INSTALLED.lock().unwrap();
    if *installed {
        output!("Fixup already installed");
        return Ok(());
    }

    let exe_path = std::env::current_exe()?;
    output!("Exe path: `{}`", exe_path.display());

    // TODO: this is a very crude match but might be good enough.
    let exe_file_data = std::fs::read(exe_path)?;
    let exe_size = exe_file_data.len();

    output!("Exe size: {}", exe_size);

    match exe_size {
        // Mech3 v1.2
        2384384 => crate::mech3::install()?,
        // Recoil
        1254912 | 1868288 => crate::recoil::install()?,
        _ => {
            output!("ERROR: Exe unknown");
        }
    }

    *installed = true;
    output!("Fixup installed");
    Ok(())
}
