pub use memmap2::Mmap;
pub use std::fs::File;
pub use std::time::Instant;

pub const FILE_PATH: &str = "../1brc/measurements.txt";

pub const SEMI_COLON_BYTE: u8 = 59;
pub const NEWLINE_BYTE: u8 = 10;
pub const MI_B_100: usize = 1024 * 1024 * 100;
pub const GIB: usize = 1024 * 1024 * 1024;

pub fn print_progress_timing(desc: &str, time: Instant, total_time: Instant) -> Instant {
    println!(
        "===> {desc:>25}: {time:>15} | {total_time:>8.5} s",
        time = format!("{:?}", time.elapsed()),
        total_time = total_time.elapsed().as_secs_f32()
    );
    Instant::now()
}
