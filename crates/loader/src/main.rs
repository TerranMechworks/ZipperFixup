use color_eyre::eyre::{Result, WrapErr as _, eyre};
use std::ffi::c_void;
use std::fmt;
use tracing::{debug, error};
use windows::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, HMODULE, WIN32_ERROR};
use windows::Win32::System::Diagnostics::Debug::{
    IMAGE_FILE_DLL, IMAGE_NT_HEADERS32, ReadProcessMemory, WriteProcessMemory,
};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEM_RESERVE, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READWRITE, PAGE_GUARD,
    PAGE_NOACCESS, PAGE_PROTECTION_FLAGS, PAGE_READWRITE, VirtualAlloc, VirtualAllocEx,
    VirtualQueryEx,
};
use windows::Win32::System::SystemServices::{
    IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE,
};
use windows::Win32::System::Threading::{
    CREATE_DEFAULT_ERROR_MODE, CREATE_SUSPENDED, CreateProcessW, CreateRemoteThread,
    GetExitCodeProcess, INFINITE, OpenProcess, PROCESS_ALL_ACCESS, PROCESS_INFORMATION,
    ResumeThread, STARTUPINFOW, WaitForSingleObject,
};
use windows::core::{Error, HSTRING, s};

fn main() -> Result<()> {
    color_eyre::install()?;

    let fmt_event = tracing_subscriber::fmt::format()
        .with_ansi(true)
        .with_level(true)
        .with_source_location(false)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .without_time()
        .pretty();
    tracing_subscriber::fmt().event_format(fmt_event).init();

    let app_name = "RecoilD3D.exe";
    launch(app_name)
}

fn launch(app_name: &str) -> Result<()> {
    let application_name = HSTRING::from(app_name);

    let mut si = STARTUPINFOW::default();
    si.cb = size_of::<STARTUPINFOW>() as _;
    let mut pi = PROCESS_INFORMATION::default();

    debug!("creating process `{app_name}`");
    unsafe {
        CreateProcessW(
            &application_name,
            None,
            None,
            None,
            false,
            CREATE_SUSPENDED | CREATE_DEFAULT_ERROR_MODE,
            None,
            None,
            &si,
            &mut pi,
        )
    }
    .wrap_err("failed to create process")?;

    let hprocess = pi.hProcess;

    update_process_with_dll(hprocess).wrap_err("failed to update process")?;

    debug!("resuming process");
    let suspend_count = unsafe { ResumeThread(pi.hThread) };
    if suspend_count == u32::MAX {
        let e = Error::from_win32();
        return Err(e).wrap_err("failed to resume process");
    }

    debug!("waiting for process...");
    let _wait_event = unsafe { WaitForSingleObject(hprocess, INFINITE) };

    debug!("process ended");
    let mut exit_code = 0;
    unsafe { GetExitCodeProcess(hprocess, &mut exit_code) }?;
    debug!("process exit code: {exit_code}");

    Ok(())
}

fn update_process_with_dll(hprocess: HANDLE) -> Result<(), InjectError> {
    let addr = find_exe_module(hprocess)?.ok_or(InjectError::ExeModuleNotFound)?;

    // UpdateImports32(hProcess, hModule, rlpDlls, nDlls)
    Ok(())
}

const PAGE_ACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0xFF);

fn find_exe_module(hprocess: HANDLE) -> Result<Option<usize>, InjectError> {
    const MBI_LEN: usize = size_of::<MEMORY_BASIC_INFORMATION>();
    let mut mbi = MEMORY_BASIC_INFORMATION::default();
    let mut address = 0x10000;

    loop {
        debug!("querying memory at 0x{address:08x}");

        let lpaddress = Some(address as *const _);
        let count = unsafe { VirtualQueryEx(hprocess, lpaddress, &mut mbi, MBI_LEN) };
        if count == 0 {
            return Ok(None);
        }

        // ignore unaligned region at the end of the address space
        if (mbi.RegionSize & 0xfff) == 0xfff {
            return Ok(None);
        }

        let next = (mbi.BaseAddress as usize) + mbi.RegionSize;
        if next < address {
            return Ok(None);
        }

        let curr = std::mem::replace(&mut address, next);

        let uncommitted = mbi.State != MEM_COMMIT;
        let no_access = (mbi.Protect & PAGE_ACCESS) == PAGE_NOACCESS;
        let guard_page = mbi.Protect.contains(PAGE_GUARD);
        if uncommitted || no_access || guard_page {
            continue;
        }

        // TODO: cbread check
        let mut idh = IMAGE_DOS_HEADER::default();
        {
            let lpbaseaddress = curr as _;
            let lpbuffer: *mut IMAGE_DOS_HEADER = &mut idh;
            let nsize = size_of::<IMAGE_DOS_HEADER>();
            unsafe { ReadProcessMemory(hprocess, lpbaseaddress, lpbuffer as _, nsize, None) }
                .map_err(InjectError::ReadProcessMemory)?;
        }

        if idh.e_magic != IMAGE_DOS_SIGNATURE {
            return Err(InjectError::SignatureDos);
        }

        let lfanew = idh.e_lfanew as usize;
        if lfanew > mbi.RegionSize || lfanew < size_of::<IMAGE_DOS_HEADER>() {
            return Err(InjectError::ExeFormat);
        }

        // TODO: cbread check
        let mut inh = IMAGE_NT_HEADERS32::default();
        {
            let lpbaseaddress = (curr + lfanew) as _;
            let lpbuffer: *mut IMAGE_NT_HEADERS32 = &mut inh;
            let nsize = size_of::<IMAGE_NT_HEADERS32>();
            unsafe { ReadProcessMemory(hprocess, lpbaseaddress, lpbuffer as _, nsize, None) }
                .map_err(InjectError::ReadProcessMemory)?;
        }

        if inh.Signature != IMAGE_NT_SIGNATURE {
            return Err(InjectError::SignatureNt);
        }

        // inh.OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR_MAGIC_32

        let machine = inh.FileHeader.Machine.0;
        let magic = inh.OptionalHeader.Magic.0;
        debug!("module addr={curr:08x} machine={machine:04x} magic={magic:04x}");

        let is_exe = !inh.FileHeader.Characteristics.contains(IMAGE_FILE_DLL);
        if is_exe {
            return Ok(Some(address));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum InjectError {
    SignatureDos,
    SignatureNt,
    ExeFormat,
    ExeModuleNotFound,
    ReadProcessMemory(windows::core::Error),
}

impl fmt::Display for InjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignatureDos => f.write_str("invalid exe signature (dos)"),
            Self::SignatureNt => f.write_str("invalid exe signature (nt)"),
            Self::ExeFormat => f.write_str("invalid exe format"),
            Self::ExeModuleNotFound => f.write_str("exe module not found"),
            Self::ReadProcessMemory(e) => write!(f, "failed to read memory: {}", e),
        }
    }
}

impl std::error::Error for InjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadProcessMemory(e) => Some(e),
            _ => None,
        }
    }
}
