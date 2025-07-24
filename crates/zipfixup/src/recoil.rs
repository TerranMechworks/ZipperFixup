use crate::{Result, output};

pub(crate) fn install_hooks() -> Result<()> {
    output!("Installing hooks... (RC)");

    output!("Installed hooks");
    Ok(())
}
