use crate::output;
use anyhow::{bail, Context as _, Result};
use retour::GenericDetour;
use std::cmp;
use std::sync::{Mutex, OnceLock};

// Globals

const SCREEN_WIDTH_PX: *const i32 = 0x8007cc as _;
const SCREEN_HEIGHT_PX: *const i32 = 0x8007d0 as _;
const _SCREEN_WIDTH_BYTES: *const i32 = 0x8007d4 as _;
const _SCREEN_BUF_COL_DEPTH_BYTES: *const i32 = 0x8007d8 as _;

// Hooked functions

type DrawLineFn = extern "fastcall" fn(*mut (), i32, i32, i32, i32, i16);
const DRAW_LINE_ADDR: i32 = 0x00563b80;

type DrawDashedLineFn = extern "fastcall" fn(*mut (), i32, i32, i32, i32, i16, i32);
const DRAW_DASHED_LINE_ADDR: i32 = 0x00563c50;

// Hooks

static DRAW_LINE_HOOK: OnceLock<GenericDetour<DrawLineFn>> = OnceLock::new();
static DRAW_DASHED_LINE_HOOK: OnceLock<GenericDetour<DrawDashedLineFn>> = OnceLock::new();
static INSTALLED: Mutex<bool> = Mutex::new(false);

pub fn install_hooks() -> Result<()> {
    output!("Installing hooks (MW)");
    let mut installed = INSTALLED.lock().unwrap();
    if *installed {
        return Ok(());
    }

    let draw_line_detour = unsafe {
        let target: DrawLineFn = std::mem::transmute(DRAW_LINE_ADDR);
        GenericDetour::new(target, draw_line)
    }
    .context("failed to detour DrawLine")?;

    unsafe { draw_line_detour.enable() }.context("failed to enable DrawLine")?;

    if let Err(_) = DRAW_LINE_HOOK.set(draw_line_detour) {
        bail!("failed to set DrawLine hook");
    }

    output!("Hooked DrawLine");

    let draw_dashed_line_detour = unsafe {
        let target: DrawDashedLineFn = std::mem::transmute(DRAW_DASHED_LINE_ADDR);
        GenericDetour::new(target, draw_dashed_line)
    }
    .context("failed to detour DrawDashedLine")?;

    unsafe { draw_dashed_line_detour.enable() }.context("failed to enable DrawDashedLine")?;

    if let Err(_) = DRAW_DASHED_LINE_HOOK.set(draw_dashed_line_detour) {
        bail!("failed to set DrawDashedLine hook");
    }

    output!("Hooked DrawDashedLine");

    *installed = true;
    Ok(())
}

fn clamp(min: i32, max: i32, value: i32) -> i32 {
    cmp::min(max, cmp::max(value, min))
}

fn clamp_to_screen_space(x: i32, y: i32) -> (i32, i32) {
    let x_res = unsafe { SCREEN_WIDTH_PX.read() };
    let y_res = unsafe { SCREEN_HEIGHT_PX.read() };
    let x_out = clamp(0, x_res - 1, x);
    let y_out = clamp(0, y_res - 1, y);
    (x_out, y_out)
}

extern "fastcall" fn draw_line(
    screen_buffer: *mut (),
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: i16,
) {
    output!("draw_line ({}, {}) - ({}, {})", x1, y1, x2, y2);

    let (x1, y1) = clamp_to_screen_space(x1, y1);
    let (x2, y2) = clamp_to_screen_space(x2, y2);

    let draw_line_hook = DRAW_LINE_HOOK.get().unwrap();
    draw_line_hook.call(screen_buffer, x1, y1, x2, y2, color)
}

extern "fastcall" fn draw_dashed_line(
    screen_buffer: *mut (),
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: i16,
    sections: i32,
) {
    output!("draw_dashed_line ({}, {}) - ({}, {})", x1, y1, x2, y2);

    let (x1, y1) = clamp_to_screen_space(x1, y1);
    let (x2, y2) = clamp_to_screen_space(x2, y2);

    let draw_dashed_line_hook = DRAW_DASHED_LINE_HOOK.get().unwrap();
    draw_dashed_line_hook.call(screen_buffer, x1, y1, x2, y2, color, sections)
}
