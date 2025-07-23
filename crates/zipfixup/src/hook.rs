use crate::{Result, output};
use retour::{Function, GenericDetour, HookableWith};
use std::sync::OnceLock;

pub(crate) fn hook<T, D>(
    name: &'static str,
    target: T,
    func: D,
    hook: &OnceLock<GenericDetour<T>>,
) -> Result<()>
where
    T: HookableWith<D> + Sized,
    D: Function,
{
    output!("Hooking {}...", name);

    let detour = unsafe { GenericDetour::new(target, func) }
        .map_err(|e| format!("failed to detour {}: {}", name, e))?;

    unsafe { detour.enable() }.map_err(|e| format!("failed to enable {}: {}", name, e))?;

    hook.set(detour)
        .map_err(|_| format!("failed to set {} hook: already set", name))?;

    output!("Hooked {}", name);
    Ok(())
}

macro_rules! decl_hooks {
    ($($name:ident: $type:ty,)+) => {
        $(static $name: OnceLock<GenericDetour<$type>> = OnceLock::new();)+
    };
}
pub(crate) use decl_hooks;
