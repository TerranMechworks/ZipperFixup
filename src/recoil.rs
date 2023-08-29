use crate::output;
use anyhow::{bail, Context as _, Result};
use retour::GenericDetour;
use std::sync::{Mutex, OnceLock};

// Globals

const GAME_OPTIONS_VMODE: *const *mut i32 = 0x004e5d30 as _;

const REPLICATE: *mut i32 = 0x00632128 as _;
const BIT_DEPTH: *mut i32 = 0x00632150 as _;

const RENDER_W: *mut i32 = 0x00632200 as _;
const RENDER_H: *mut i32 = 0x00632204 as _;

const DISPLAY_W: *mut i32 = 0x00632220 as _;
const DISPLAY_H: *mut i32 = 0x00632224 as _;

const WINDOW_W: *mut i32 = 0x00632240 as _;
const WINDOW_H: *mut i32 = 0x00632244 as _;

// Called function

type SetGameOptionsPosFn = extern "fastcall" fn(top: i32, left: i32);
const SET_GAME_OPTIONS_RENDER_POS_ADDR: *const SetGameOptionsPosFn = 0x00408530 as _;
const SET_GAME_OPTIONS_WINDOW_POS_ADDR: *const SetGameOptionsPosFn = 0x00408700 as _;
const SET_GAME_OPTIONS_DISPLAY_POS_ADDR: *const SetGameOptionsPosFn = 0x004085e0 as _;

type SetGameOptionsResFn = extern "fastcall" fn(width: i32, height: i32);
const SET_GAME_OPTIONS_RENDER_RES_ADDR: *const SetGameOptionsResFn = 0x00408500 as _;
const SET_GAME_OPTIONS_WINDOW_RES_ADDR: *const SetGameOptionsResFn = 0x004086e0 as _;
const SET_GAME_OPTIONS_DISPLAY_RES_ADDR: *const SetGameOptionsResFn = 0x00408620 as _;

type SetGameOptionsInt32Fn = extern "fastcall" fn(i32);
const SET_GAME_OPTIONS_DISPLAY_FIELD8_ADDR: *const SetGameOptionsInt32Fn = 0x00408680 as _;
const SET_GAME_OPTIONS_REPLICATE_ADDR: *const SetGameOptionsInt32Fn = 0x00408300 as _;

// Hooked functions

type SetResolutionFn = extern "fastcall" fn(vmode: i32);
const SET_LAUNCHER_RES_ADDR: i32 = 0x00408720;
const SET_GAME_RES_ADDR: i32 = 0x004a7990;

// Hooks

static SET_LAUNCHER_RES_HOOK: OnceLock<GenericDetour<SetResolutionFn>> = OnceLock::new();
static SET_GAME_RES_HOOK: OnceLock<GenericDetour<SetResolutionFn>> = OnceLock::new();
static INSTALLED: Mutex<bool> = Mutex::new(false);

pub fn install_hooks() -> Result<()> {
    output!("Installing hooks (RC)");
    let mut installed = INSTALLED.lock().unwrap();
    if *installed {
        return Ok(());
    }

    let set_launcher_res_detour = unsafe {
        let target: SetResolutionFn = std::mem::transmute(SET_LAUNCHER_RES_ADDR);
        GenericDetour::new(target, set_launcher_res)
    }
    .context("failed to detour SetLauncherRes")?;

    unsafe { set_launcher_res_detour.enable() }.context("failed to enable SetLauncherRes")?;

    if let Err(_) = SET_LAUNCHER_RES_HOOK.set(set_launcher_res_detour) {
        bail!("failed to set SetLauncherRes hook");
    }

    output!("Hooked SetLauncherRes");

    let set_game_res_detour = unsafe {
        let target: SetResolutionFn = std::mem::transmute(SET_GAME_RES_ADDR);
        GenericDetour::new(target, set_game_res)
    }
    .context("failed to detour SetGameRes")?;

    unsafe { set_game_res_detour.enable() }.context("failed to enable SetGameRes")?;

    if let Err(_) = SET_GAME_RES_HOOK.set(set_game_res_detour) {
        bail!("failed to set SetGameRes hook");
    }

    output!("Hooked SetGameRes");

    *installed = true;
    Ok(())
}

fn set_res_full(width: i32, height: i32) {
    output!("set_res_full: {} x {}", width, height);

    let render_pos = unsafe { *SET_GAME_OPTIONS_RENDER_POS_ADDR };
    render_pos(0, 0);
    let render_res = unsafe { *SET_GAME_OPTIONS_RENDER_RES_ADDR };
    render_res(width, height);

    let window_pos = unsafe { *SET_GAME_OPTIONS_WINDOW_POS_ADDR };
    window_pos(0, 0);
    let window_res = unsafe { *SET_GAME_OPTIONS_WINDOW_RES_ADDR };
    window_res(width, height);

    let display_pos = unsafe { *SET_GAME_OPTIONS_DISPLAY_POS_ADDR };
    display_pos(0, 0);
    let display_res = unsafe { *SET_GAME_OPTIONS_DISPLAY_RES_ADDR };
    display_res(width, height);

    let display_f8 = unsafe { *SET_GAME_OPTIONS_DISPLAY_FIELD8_ADDR };
    display_f8(16);
    let replicate = unsafe { *SET_GAME_OPTIONS_REPLICATE_ADDR };
    replicate(0);
}

fn set_res_half(width: i32, height: i32) {
    output!("set_res_half: {} x {}", width, height);

    let render_pos = unsafe { *SET_GAME_OPTIONS_RENDER_POS_ADDR };
    render_pos(0, 0);
    let render_res = unsafe { *SET_GAME_OPTIONS_RENDER_RES_ADDR };
    render_res(width / 2, height / 2);

    let window_pos = unsafe { *SET_GAME_OPTIONS_WINDOW_POS_ADDR };
    window_pos(0, 0);
    let window_res = unsafe { *SET_GAME_OPTIONS_WINDOW_RES_ADDR };
    window_res(width, height);

    let display_pos = unsafe { *SET_GAME_OPTIONS_DISPLAY_POS_ADDR };
    display_pos(0, 0);
    let display_res = unsafe { *SET_GAME_OPTIONS_DISPLAY_RES_ADDR };
    display_res(width, height);

    let display_f8 = unsafe { *SET_GAME_OPTIONS_DISPLAY_FIELD8_ADDR };
    display_f8(16);
    let replicate = unsafe { *SET_GAME_OPTIONS_REPLICATE_ADDR };
    replicate(1);
}

fn set_game_options_vmode(vmode: i32) {
    unsafe { (*GAME_OPTIONS_VMODE).write(vmode) };
}

extern "fastcall" fn set_launcher_res(vmode: i32) {
    output!("SetLauncherRes: {}", vmode);
    let hook = SET_LAUNCHER_RES_HOOK.get().unwrap();
    hook.call(vmode);
    // match vmode {
    //     2 => {
    //         set_game_options_vmode(2);
    //         set_res_half(640, 400);
    //     }
    //     3 => {
    //         set_game_options_vmode(3);
    //         set_res_half(640, 480);
    //     }
    //     4 => {
    //         set_game_options_vmode(4);
    //         set_res_full(640, 400);
    //     }
    //     5 => {
    //         set_game_options_vmode(5);
    //         set_res_full(640, 480);
    //     }
    //     6 => {
    //         set_game_options_vmode(6);
    //         set_res_full(800, 600);
    //     }
    //     7 => {
    //         set_game_options_vmode(7);
    //         set_res_full(1024, 768);
    //     }
    //     8 => {
    //         set_game_options_vmode(8);
    //         set_res_full(1920, 1080);
    //     }
    //     _ => {
    //         set_game_options_vmode(0);
    //     }
    // }
}

fn set_game_res_half(width: i32, height: i32) {
    output!("set_game_res_half: {} x {}", width, height);

    unsafe { REPLICATE.write(1) };

    unsafe { RENDER_W.write(width / 2) };
    unsafe { RENDER_H.write(height / 2) };

    unsafe { DISPLAY_W.write(width) };
    unsafe { DISPLAY_H.write(height) };

    unsafe { WINDOW_W.write(width) };
    unsafe { WINDOW_H.write(height) };

    unsafe { BIT_DEPTH.write(16) };
}

fn set_game_res_full(width: i32, height: i32) {
    output!("set_game_res_full: {} x {}", width, height);

    unsafe { REPLICATE.write(0) };

    unsafe { RENDER_W.write(width) };
    unsafe { RENDER_H.write(height) };

    unsafe { DISPLAY_W.write(width) };
    unsafe { DISPLAY_H.write(height) };

    unsafe { WINDOW_W.write(width) };
    unsafe { WINDOW_H.write(height) };

    unsafe { BIT_DEPTH.write(16) };
}

fn debug_game_res() {
    let replicate = unsafe { REPLICATE.read() };
    output!("game res replicate: {}", replicate);

    let width = unsafe { RENDER_W.read() };
    let height = unsafe { RENDER_H.read() };
    output!("game res render: {} x {}", width, height);

    let width = unsafe { DISPLAY_W.read() };
    let height = unsafe { DISPLAY_H.read() };
    output!("game res display: {} x {}", width, height);

    let width = unsafe { WINDOW_W.read() };
    let height = unsafe { WINDOW_H.read() };
    output!("game res window: {} x {}", width, height);

    let bit_depth = unsafe { BIT_DEPTH.read() };
    output!("game res bit depth: {}", bit_depth);
}

extern "fastcall" fn set_game_res(vmode: i32) {
    output!("SetGameRes: {}", vmode);
    debug_game_res();
    let hook = SET_GAME_RES_HOOK.get().unwrap();
    hook.call(vmode);
    debug_game_res();
    match vmode {
        2 => set_game_res_half(640, 400),
        3 => set_game_res_half(640, 480),
        4 => set_game_res_full(640, 400),
        5 => set_game_res_full(640, 480),
        6 => set_game_res_full(800, 600),
        7 => set_game_res_full(1024, 768),
        8 => set_game_res_full(1920, 1080),
        _ => {}
    }
}
