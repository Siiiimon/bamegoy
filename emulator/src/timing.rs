use std::time::{Duration, Instant};

const CPU_HZ: f64 = 4_194_304.0;
const CYCLES_PER_LINE: u64 = 456;

const LINES_PER_TICK: u64 = 24;
pub const CYCLES_PER_SLICE: u64 = CYCLES_PER_LINE * LINES_PER_TICK;
const SLICE_TIME: f64 = CYCLES_PER_SLICE as f64 / CPU_HZ;

const MIN_SLEEP: Duration = Duration::from_micros(500);

pub fn calculate_drift(initial: Instant) -> f64 {
    SLICE_TIME - initial.elapsed().as_secs_f64()
}

pub fn nap(drift: &mut f64) {
    if *drift > 0.0 {
        
        // sleep the coarse part
        let to_sleep = drift.min(0.050); // cap to avoid long naps
        if to_sleep >= MIN_SLEEP.as_secs_f64() {
            std::thread::sleep(Duration::from_secs_f64(to_sleep));
            *drift -= to_sleep;
        }
        // finish the tiny tail with a short spin for accuracy
        let end = Instant::now() + Duration::from_secs_f64(drift.max(0.0));
        while Instant::now() < end {
            std::hint::spin_loop();
        }
        *drift = 0.0;
    }
}