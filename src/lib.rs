#![cfg(windows)]
use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};

use winapi::um::winnt::{DLL_PROCESS_ATTACH,DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH};
use std::time::Instant;

use std::ffi::CString;
use winapi::um::debugapi::OutputDebugStringA;

use std::fs;

mod mech3;

fn _debug_print(string: &str){
    let msg = CString::new(string).unwrap();

    unsafe {
        OutputDebugStringA(msg.as_ptr());
    }
}


// TODO work out how to hide this symbol but still get correct linkage
#[no_mangle]
unsafe extern "system" fn get_tick_count() -> DWORD
{
    //TODO switch to std::lazy::OnceCell?;
    static mut START_TIME : Option<Instant> = None;

    if START_TIME == None{
        START_TIME = Some(Instant::now());
    }

    let elapsed = START_TIME.unwrap().elapsed().as_millis();
    elapsed as u32
}


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{
    match call_reason {
        DLL_PROCESS_ATTACH => on_thread_attach(),
        DLL_PROCESS_DETACH => (),
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        _ => ()
    }
    minwindef::TRUE
}

fn get_exe_size() -> usize{
    let path = std::env::current_exe().unwrap();
    println!("Running with binary {}", path.to_str().unwrap());
    let exe_file_data = fs::read(path).unwrap();
    exe_file_data.len()
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ExeType{
    Unknown,
    Mech3,
}

fn lookup_exe(size: usize) -> ExeType {
    // TODO this is a very crude match but might be good enough.
    match size {
        // Mech3 1.2.
        2384384 => ExeType::Mech3,
        _ => ExeType::Unknown
    }
}

fn install_hooks(exe_type: ExeType){
    match exe_type{
        ExeType::Unknown => (),
        ExeType::Mech3 => (mech3::install_hooks()),
    }
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
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

fn patch_binary(exe_name: &str){
    const ORIG_DLL_NAME : &[u8] = b"KERNEL32.dll";
    const OLD_DLL_NAME : &[u8] = b"MECH3FIX.dll";
    const NEW_DLL_NAME : &[u8] = b"ZIPFIXUP.dll";

    println!("Checking for {}", exe_name);
    let try_read_file = fs::read(exe_name);
    let mut file_data = match try_read_file{
        Ok(res) => res,
        // Assume a failure to read means the exe isn't there
        Err(_err) => return,
    };
    println!("{} loaded.", exe_name);

    let alread_patched = replace_slice(file_data.as_mut_slice(), NEW_DLL_NAME, NEW_DLL_NAME);

    // Patch for kernel32.dll and mech3fix.dll as some people have renamed the exes
    let count = replace_slice(file_data.as_mut_slice(), ORIG_DLL_NAME, NEW_DLL_NAME) +
        replace_slice(file_data.as_mut_slice(), OLD_DLL_NAME, NEW_DLL_NAME);


    if count == 1 {
        // There should only be one instance
        println!("{} patched.", exe_name);

        let mut new_exe_name : String = exe_name[0..exe_name.len()-4].to_owned();
        new_exe_name.push_str("fixup.exe");

        let res = fs::write(&new_exe_name, file_data);
        match res {
            Ok(()) => println!("{} written.", new_exe_name),
            Err(err) =>  println!("{} failure during write: {}.", exe_name, err),
        }
    } else if alread_patched == 1 {
        println!("{} already patched.", exe_name);
    } else {
        println!("{} patching failed.", exe_name);
    }
}

#[no_mangle]
pub extern "system" fn PatchGame()
{
    const SUPPORTED_EXES : &[&str] = &["Mech3.exe", "Recoil.exe", "Recoil3dfx.exe"];
    println!();
    println!("Finding exes to patch.");

    for name in SUPPORTED_EXES.iter(){
        patch_binary(name)
    }
}