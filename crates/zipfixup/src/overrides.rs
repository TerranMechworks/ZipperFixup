use std::sync::OnceLock;
use std::time::Instant;

static START_TIME: OnceLock<Instant> = OnceLock::new();

/// Custom `GetTickCount` implementation for two reasons:
/// 1. The machine's uptime no longer matters, as the time base is when the DLL
///    was loaded.
/// 2. The resolution of [GetTickCount] is 10-16 milliseconds, which is too
///    coarse on modern/fast systems. Currently, Rust's [`Instant`] uses
///    [QueryPerformanceCounter], which has a higher resolution and also is
///    monotonic.
///
/// Note that because the output is a [`u32`], it can still wrap once the
/// executable has been running for 49.7 days.
///
/// [GetTickCount]: https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount
/// [QueryPerformanceCounter]: https://learn.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn GetTickCount() -> u32 {
    let start_time = START_TIME.get_or_init(Instant::now);
    let elapsed = start_time.elapsed().as_millis();
    elapsed as u32
}
