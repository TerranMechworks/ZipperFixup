use std::sync::OnceLock;
use std::time::Instant;

static START_TIME: OnceLock<Instant> = OnceLock::new();

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn GetTickCount() -> u32 {
    let start_time = START_TIME.get_or_init(Instant::now);
    let elapsed = start_time.elapsed().as_millis();
    elapsed as u32
}
