use crate::err_msg::ERR_MSG_ADDR;
use crate::{Result, output, patch};

const ERR_MSG_CALLS: &[(u32, u32)] = &[
    (0x004c2900, u32::from_be(0x7b_25_f4_ff)), // Interp: "No current node to enable cycle textures..."
    (0x004c294a, u32::from_be(0x31_25_f4_ff)), // Interp: "ERROR no GFX data for cycled texture (%s)"
    (0x004c2b02, u32::from_be(0x79_23_f4_ff)), // Interp: "Node (%s) has no graphics data for cycled texture\n"
];

pub(crate) fn install() -> Result<()> {
    output!("Installing... (RC)");

    for (call_site, expected_rel) in ERR_MSG_CALLS.iter().copied() {
        if let Err(e) = patch::call(call_site, ERR_MSG_ADDR, expected_rel) {
            output!("{}", e);
        }
    }

    output!("Installed");
    Ok(())
}
