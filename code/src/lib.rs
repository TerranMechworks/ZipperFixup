#![cfg(windows)]
use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};

use winapi::um::winnt::{DLL_PROCESS_ATTACH,DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH};
use std::time::Instant;

use std::ffi::CString;
use winapi::um::debugapi::OutputDebugStringA;

use sha1::Sha1;
use std::fs;
use regex::{Regex, bytes};

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

fn hash_exe() -> String{
    let path = std::env::current_exe().unwrap();
    println!("Running with binary {}", path.to_str().unwrap());
    let exe_file_data = fs::read(path).unwrap();
    let hash = Sha1::from(exe_file_data).digest();
    hash.to_string()
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ExeType{
    Unknown,
    Mech3,
}

fn lookup_exe(hash: String) -> ExeType {
    match hash.as_str() {
        // Mech3 1.2.
        "6be974b58e2303c203c12c4688e71f526a2cd8d1" => ExeType::Mech3,
        // Mech3 1.2 patched to load the dll.
        "4b71436b8cb8915423a12dcd3fc341f9073ae3c4" => ExeType::Mech3,
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
    let exe_hash = hash_exe();

    println!("exe sha {}", exe_hash);
    let exe_type = lookup_exe(exe_hash);

    println!("exe type {:?}", exe_type);

    if exe_type != ExeType::Unknown {
        println!("Installing hooks");
        install_hooks(exe_type);
    }

    // stdout closed after this
}

// TODO using regex for string replacement is overkill and adds 700k to the binary
// Maybe just use splicing.
fn patch_binary(exe_name: &str){
    const ORIG_DLL_NAME : &str = "KERNEL32.dll";
    const NEW_DLL_NAME : &[u8] = b"ZIPFIXUP.dll";

    println!("Checking for {}", exe_name);
    let try_read_file = fs::read(exe_name);
    let file_data = match try_read_file{
        Ok(res) => res,
        Err(_err) => return,
    };
    println!("{} loaded.", exe_name);

    let file_re = bytes::Regex::new(ORIG_DLL_NAME).unwrap();
    let new_file_data = file_re.replace_all(&file_data, NEW_DLL_NAME);

    //TODO check replacements were made
    println!("{} patched.", exe_name);

    let exe_re = Regex::new(r"\.exe$").unwrap();
    let new_exe_name = exe_re.replace(exe_name, "fixup.exe").to_string();
    let _res = fs::write(&new_exe_name, new_file_data);

    println!("{} written.", new_exe_name);

    //TODO check res isn't an error
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