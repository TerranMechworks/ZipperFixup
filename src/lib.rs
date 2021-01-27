#![feature(global_asm)]

#![cfg(windows)]
use std::ffi::CString;
use std::fs;
use std::time::Instant;
use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::debugapi::OutputDebugStringA;
use winapi::um::winnt::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};

mod exports;
mod mech3;

fn _debug_print(string: &str) {
    let msg = CString::new(string).unwrap();

    unsafe {
        OutputDebugStringA(msg.as_ptr());
    }
}

// TODO work out how to hide this symbol but still get correct linkage
#[no_mangle]
unsafe extern "system" fn get_tick_count() -> DWORD {
    //TODO switch to std::lazy::OnceCell?;
    static mut START_TIME: Option<Instant> = None;

    if START_TIME == None {
        START_TIME = Some(Instant::now());
    }

    let elapsed = START_TIME.unwrap().elapsed().as_millis();
    elapsed as u32
}

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
    minwindef::TRUE
}

fn get_exe_size() -> usize {
    let path = std::env::current_exe().unwrap();
    println!("Running with binary {}", path.to_str().unwrap());
    let exe_file_data = fs::read(path).unwrap();
    exe_file_data.len()
}

#[derive(Debug, PartialEq)]
enum ExeType {
    Unknown,
    Mech3,
}

fn lookup_exe(size: usize) -> ExeType {
    // TODO this is a very crude match but might be good enough.
    match size {
        // Mech3 1.2.
        2384384 => ExeType::Mech3,
        _ => ExeType::Unknown,
    }
}

fn install_hooks(exe_type: ExeType) {
    match exe_type {
        ExeType::Unknown => (),
        ExeType::Mech3 => mech3::install_hooks(),
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn on_thread_attach() {
    println!("Fixup dll loaded");
    println!("Version:{}", VERSION);
    let exe_hash = get_exe_size();

    println!("exe size {}", exe_hash);
    let exe_type = lookup_exe(exe_hash);

    println!("exe type {:?}", exe_type);

    if exe_type != ExeType::Unknown {
        println!("Installing hooks");
        install_hooks(exe_type);
    }

    // stdout closed after this
}

fn replace_slice<T>(buf: &mut [T], from: &[T], to: &[T]) -> usize
where
    T: Clone + PartialEq,
{
    let mut count: usize = 0;
    for i in 0..=buf.len() - from.len() {
        if buf[i..].starts_with(from) {
            count += 1;
            buf[i..(i + from.len())].clone_from_slice(to);
        }
    }
    count
}

fn patch_binary(exe_name: &str) {
    const ORIG_DLL_NAME: &[u8] = b"KERNEL32.dll";
    const OLD_DLL_NAME: &[u8] = b"MECH3FIX.dll";
    const NEW_DLL_NAME: &[u8] = b"ZIPFIXUP.dll";

    println!("Checking for {}", exe_name);
    let mut file_data = match fs::read(exe_name) {
        Ok(res) => res,
        // Assume a failure to read means the exe isn't there
        Err(_) => return,
    };
    println!("{} loaded.", exe_name);

    // TODO: Is this check worth it? Is it still worth patching in this case?
    let already_patched = file_data
        .windows(NEW_DLL_NAME.len())
        .any(|window| window == NEW_DLL_NAME);

    // Patch for kernel32.dll and mech3fix.dll as some people have renamed the exes
    let count = replace_slice(file_data.as_mut_slice(), ORIG_DLL_NAME, NEW_DLL_NAME)
        + replace_slice(file_data.as_mut_slice(), OLD_DLL_NAME, NEW_DLL_NAME);

    if count == 1 {
        // There should only be one instance
        println!("{} patched.", exe_name);

        let exe_base = exe_name.strip_suffix(".exe").unwrap_or(exe_name);
        let new_name = format!("{}fixup.exe", exe_base);

        match fs::write(&new_name, file_data) {
            Ok(()) => println!("{} written.", new_name),
            Err(err) => println!("{} failure during write: {}.", exe_name, err),
        }
    } else if already_patched {
        println!("{} already patched.", exe_name);
    } else {
        println!("{} patching failed.", exe_name);
    }
}

#[no_mangle]
pub extern "system" fn PatchGame() {
    println!();
    println!("Finding exes to patch.");

    for name in &["Mech3.exe", "Recoil.exe", "Recoil3dfx.exe"] {
        patch_binary(name)
    }
}
