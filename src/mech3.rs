use crate::hook::{decl_hooks, hook};
use crate::{Result, output};
use retour::GenericDetour;
use std::sync::OnceLock;

// Globals

const SCREEN_WIDTH_PX: *const i32 = 0x8007cc as _;
const SCREEN_HEIGHT_PX: *const i32 = 0x8007d0 as _;

// Hooked functions

type DrawLineFn = extern "fastcall" fn(*mut (), i32, i32, i32, i32, i16);
const DRAW_LINE_ADDR: i32 = 0x00563b80;

type DrawDashedLineFn = extern "fastcall" fn(*mut (), i32, i32, i32, i32, i16, i32);
const DRAW_DASHED_LINE_ADDR: i32 = 0x00563c50;

// Hooks

decl_hooks! {
    DRAW_LINE_HOOK: DrawLineFn,
    DRAW_DASHED_LINE_HOOK: DrawDashedLineFn,
}

pub(crate) fn install_hooks() -> Result<()> {
    output!("Installing hooks... (MW)");

    let target: DrawLineFn = unsafe { std::mem::transmute(DRAW_LINE_ADDR) };
    hook("DrawLine", target, draw_line, &DRAW_LINE_HOOK)?;

    let target: DrawDashedLineFn = unsafe { std::mem::transmute(DRAW_DASHED_LINE_ADDR) };
    hook(
        "DrawDashedLine",
        target,
        draw_dashed_line,
        &DRAW_DASHED_LINE_HOOK,
    )?;

    output!("Installed hooks");
    Ok(())
}

fn clamp(min: i32, max: i32, value: i32) -> i32 {
    std::cmp::min(max, std::cmp::max(value, min))
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
    // output!("draw_line ({}, {}) - ({}, {})", x1, y1, x2, y2);

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
    // output!("draw_dashed_line ({}, {}) - ({}, {})", x1, y1, x2, y2);

    let (x1, y1) = clamp_to_screen_space(x1, y1);
    let (x2, y2) = clamp_to_screen_space(x2, y2);

    let draw_dashed_line_hook = DRAW_DASHED_LINE_HOOK.get().unwrap();
    draw_dashed_line_hook.call(screen_buffer, x1, y1, x2, y2, color, sections)
}
