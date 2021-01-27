use std::cmp;
use std::thread;
use std::time::Duration;

macro_rules! addr_as_fn {
    ($address:expr, $t:ty) => {
        unsafe { std::mem::transmute::<*const (), $t>($address as _) }
    };
}

const SCREEN_WIDTH_PX: *const i32 = 0x8007cc as *const i32;
const SCREEN_HEIGHT_PX: *const i32 = 0x8007d0 as *const i32;
const _SCREEN_WIDTH_BYTES: *const i32 = 0x8007d4 as *const i32;
const _SCREEN_BUF_COL_DEPTH_BYTES: *const i32 = 0x8007d8 as *const i32;

fn clamp(min: i32, max: i32, value: i32) -> i32 {
    cmp::min(max, cmp::max(value, min))
}

fn clamp_to_screen_space(x: i32, y: i32) -> (i32, i32) {
    let x_res = unsafe { *SCREEN_WIDTH_PX };
    let y_res = unsafe { *SCREEN_HEIGHT_PX };
    let x_out = clamp(0, x_res - 1, x);
    let y_out = clamp(0, y_res - 1, y);
    (x_out, y_out)
}

// Function pointer in game we can overwrite
type DrawLineFn = extern "fastcall" fn(*mut u16, i32, i32, i32, i32, i16);
const DRAW_LINE_FN_PTR: *mut DrawLineFn = 0x008008c0 as *mut DrawLineFn;
const DRAW_LINE_ADDR: *const () = 0x00563b80 as _;

type DrawDashedLineFn = extern "fastcall" fn(*mut u16, i32, i32, i32, i32, i16, i32);
const DRAW_DASHED_LINE_FN_PTR: *mut DrawDashedLineFn = 0x008008c4 as *mut DrawDashedLineFn;
const DRAW_DASHED_LINE_ADDR: *const () = 0x00563c50 as _;

pub fn install_hooks() {
    let _handle = thread::spawn(|| {
        // TODO Really unsafe.
        // Would it be better to replace the start of the original functions with a jmp?
        thread::sleep(Duration::from_millis(5000));

        unsafe {
            *DRAW_LINE_FN_PTR = draw_line;
            *DRAW_DASHED_LINE_FN_PTR = draw_dashed_line;
        }
    });
}

extern "fastcall" fn draw_line(
    screen_buffer: *mut u16,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: i16,
) {
    let (x1, y1) = clamp_to_screen_space(x1, y1);
    let (x2, y2) = clamp_to_screen_space(x2, y2);

    let func = addr_as_fn!(DRAW_LINE_ADDR, DrawLineFn);
    func(screen_buffer, x1, y1, x2, y2, color);
}

extern "fastcall" fn draw_dashed_line(
    screen_buffer: *mut u16,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: i16,
    sections: i32,
) {
    let (x1, y1) = clamp_to_screen_space(x1, y1);
    let (x2, y2) = clamp_to_screen_space(x2, y2);

    let func = addr_as_fn!(DRAW_DASHED_LINE_ADDR, DrawDashedLineFn);
    func(screen_buffer, x1, y1, x2, y2, color, sections);
}
