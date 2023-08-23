use crate::dbg;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::winnt::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => on_thread_attach(),
        DLL_PROCESS_DETACH => (),
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        _ => (),
    }
    TRUE
}

fn on_thread_attach() {
    dbg!("Fixup loaded ({})", VERSION);

    let path = match std::env::current_exe() {
        Ok(path) => {
            dbg!("Running with binary `{}`", path.display());
            path
        }
        Err(e) => {
            dbg!("Failed to get binary: {:?}", e);
            return;
        }
    };

    let exe_file_data = std::fs::read(path).unwrap();
    let exe_size = exe_file_data.len();

    dbg!("exe size {}", exe_size);

    // TODO: this is a very crude match but might be good enough.
    match exe_size {
        // Mech3 1.2.
        2384384 => {
            dbg!("Installing hooks (MW)");
            crate::mech3::install_hooks();
        }
        _ => {}
    }
}
