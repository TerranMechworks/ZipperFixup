use crate::{Result, output};
use std::sync::Mutex;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            output!("DLL_PROCESS_ATTACH");
            // disable DLL_THREAD_ATTACH/DLL_THREAD_DETACH notifications, since
            // we don't need them, and may help with spawning threads
            unsafe { DisableThreadLibraryCalls(dll_module) };
            // it's unclear what is allowed to be done in DllMain.
            // theoretically, even spawning a thread is not allowed:
            // https://learn.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-best-practices
            // https://devblogs.microsoft.com/oldnewthing/20070904-00/?p=25283
            let _ = std::thread::spawn(on_thread_attach);
        }
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    TRUE
}

fn on_thread_attach() {
    if let Err(e) = on_thread_attach_inner() {
        output!("FATAL ERROR: {:?}", e);
        panic!("FATAL ERROR");
    }
}

static INSTALLED: Mutex<bool> = Mutex::new(false);

fn on_thread_attach_inner() -> Result<()> {
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
        2384384 => crate::mech3::install_hooks()?,
        // Recoil
        1254912 | 1868288 => crate::recoil::install_hooks()?,
        _ => {
            output!("ERROR: Exe unknown");
        }
    }

    *installed = true;
    output!("Fixup installed");
    Ok(())
}
