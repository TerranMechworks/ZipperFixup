use crate::output;
use anyhow::{Context as _, Result};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            output!("DLL_PROCESS_ATTACH");
            unsafe { DisableThreadLibraryCalls(dll_module) };
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

fn on_thread_attach_inner() -> Result<()> {
    output!("Fixup loaded ({})", VERSION);

    let path = std::env::current_exe().context("Failed to get exe")?;
    output!("Current exe `{}`", path.display());

    let exe_file_data = std::fs::read(path).context("Failed to read exe")?;
    let exe_size = exe_file_data.len();

    // TODO: this is a very crude match but might be good enough.
    match exe_size {
        // Mech3 v1.2
        2384384 => crate::mech3::install_hooks(),
        // Recoil
        1868288 => crate::recoil::install_hooks(),
        _ => {
            output!("Unknown exe {}", exe_size);
            Ok(())
        }
    }
}
