use std::sync::OnceLock;
use std::time::Instant;
use winapi::shared::minwindef::DWORD;

static START_TIME: OnceLock<Instant> = OnceLock::new();

// TODO: work out how to hide this symbol but still get correct linkage
#[unsafe(no_mangle)]
extern "system" fn get_tick_count() -> DWORD {
    let start_time = START_TIME.get_or_init(Instant::now);
    let elapsed = start_time.elapsed().as_millis();
    elapsed as DWORD
}
