use color_eyre::eyre::{Result, WrapErr as _, eyre};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx};
use windows::Win32::System::Threading::{
    CREATE_SUSPENDED, CreateProcessW, CreateRemoteThread, INFINITE, OpenProcess,
    PROCESS_ALL_ACCESS, PROCESS_INFORMATION, ResumeThread, STARTUPINFOW, WaitForSingleObject,
};
use windows::core::{HSTRING, s};

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello, World");

    let app_name = "RecoilD3D.exe";
    launch(app_name)
}

fn launch(app_name: &str) -> Result<()> {
    let app_name = HSTRING::from(app_name);

    let startup_info = STARTUPINFOW::default();
    let mut proc_info = PROCESS_INFORMATION::default();

    unsafe {
        CreateProcessW(
            &app_name,
            None,
            None,
            None,
            false,
            CREATE_SUSPENDED,
            None,
            None,
            &startup_info,
            &mut proc_info,
        )
    }
    .wrap_err("failed to create process")?;

    let hprocess = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, proc_info.dwProcessId) }
        .wrap_err("failed to open process")?;

    let memory = unsafe {
        VirtualAllocEx(
            hprocess,
            None,
            4096,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )
    };
    if memory.is_null() {
        return Err(eyre!("failed to allocate process memory"));
    }

    {
        const FIXUP: &[u8] = b"aaa.dll\0";
        let lpbuffer = FIXUP.as_ptr() as _;
        let nsize = FIXUP.len();
        unsafe { WriteProcessMemory(hprocess, memory, lpbuffer, nsize, None) }
            .wrap_err("failed to write process memory")?;
    }

    let hmodule = unsafe { GetModuleHandleA(s!("kernel32.dll")) }
        .wrap_err("failed to get handle to `kernel32.dll`")?;

    let load_library = unsafe { GetProcAddress(hmodule, s!("LoadLibraryA")) }
        .ok_or_else(|| eyre!("failed to get address of `LoadLibraryA`"))?;

    // horrible cast
    let load_library: unsafe extern "system" fn(*mut std::ffi::c_void) -> u32 =
        unsafe { std::mem::transmute(load_library) };

    let _handle =
        unsafe { CreateRemoteThread(hprocess, None, 0, Some(load_library), Some(memory), 0, None) }
            .wrap_err("failed to inject thread")?;

    let res = unsafe { ResumeThread(proc_info.hThread) };
    if res == (-1i32 as u32) {
        return Err(eyre!("failed to resume process"));
    }

    let _res = unsafe { CloseHandle(proc_info.hThread) };
    let _res = unsafe { CloseHandle(hprocess) };

    let _evt = unsafe { WaitForSingleObject(proc_info.hProcess, INFINITE) };

    Ok(())
}
