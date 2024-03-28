pub use memmap2::Mmap;
pub use std::fs::File;
pub use std::time::Instant;

pub const FILE_PATH: &str = "../1brc/measurements.txt";

pub const SEMI_COLON_BYTE: u8 = 59;
pub const NEWLINE_BYTE: u8 = 10;

pub const FLOAT_MAX_BYTES: [u8; 4] = [57, 57, 46, 57];
pub const FLOAT_MIN_BYTES: [u8; 5] = [45, 57, 57, 46, 57];
pub const MINUS_AS_BYTE: u8 = 45;
pub const DOT_AS_BYTE: u8 = 46;
pub const ZERO_AS_BYTE: u8 = 48;

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

/// Replace the minimum value in `min_val` with `ascii_float` if `ascii_float` is less than the current minimum value.
/// return whether the minimum value is negative (whether it is replaced or not)
///
/// # Arguments
///
/// * `min_val` - The current minimum value.
/// * `min_is_neg` - Whether the current minimum value is negative.
/// * `ascii_float` - The new 2-byte ASCII float value to compare with the current minimum value.
/// * `ascii_is_neg` - Whether the new ASCII float value is negative.
#[inline(always)]
pub fn replace_min_if_2_ascii_float_less_than(
    min_val: &mut [u8; 3],
    min_is_neg: bool,
    ascii_float: [u8; 2],
    ascii_is_neg: bool,
) -> bool {
    let mut new_min_val_is_neg = min_is_neg;

    if ascii_is_neg {
        if min_is_neg {
            if min_val[0] == ZERO_AS_BYTE {
                if min_val[1] > ascii_float[0] {
                    min_val[1] = ascii_float[0];
                    min_val[2] = ascii_float[1];
                } else if min_val[1] == ascii_float[0] && min_val[2] < ascii_float[1] {
                    min_val[2] = ascii_float[1];
                }
            }
        } else {
            new_min_val_is_neg = true;
            min_val[0] = MINUS_AS_BYTE;
            min_val[1] = ascii_float[0];
            min_val[2] = ascii_float[1];
        }
    } else {
        if !min_is_neg {
            if min_val[0] == ZERO_AS_BYTE {
                if min_val[1] > ascii_float[0] {
                    min_val[1] = ascii_float[0];
                    min_val[2] = ascii_float[1];
                } else if min_val[1] == ascii_float[0] && min_val[2] > ascii_float[1] {
                    min_val[2] = ascii_float[1];
                }
            } else {
                min_val[0] = ZERO_AS_BYTE;
                min_val[1] = ascii_float[0];
                min_val[2] = ascii_float[1];
            }
        }
    }

    new_min_val_is_neg
}

#[inline(always)]
pub fn replace_min_if_3_ascii_float_less_than(
    min_val: &mut [u8; 3],
    min_is_neg: bool,
    ascii_float: [u8; 3],
    ascii_is_neg: bool,
) -> bool {
    let mut new_min_val_is_neg = min_is_neg;

    if ascii_is_neg {
        if min_is_neg {
            if min_val[0] == ZERO_AS_BYTE {
                min_val[0] = ascii_float[0];
                min_val[1] = ascii_float[1];
                min_val[2] = ascii_float[2];
            } else {
                if min_val[0] < ascii_float[0] {
                    min_val[0] = ascii_float[0];
                    min_val[1] = ascii_float[1];
                    min_val[2] = ascii_float[2];
                } else if min_val[0] == ascii_float[0] {
                    if min_val[1] < ascii_float[1] {
                        min_val[1] = ascii_float[1];
                        min_val[2] = ascii_float[2];
                    } else if min_val[1] == ascii_float[1] && min_val[2] < ascii_float[2] {
                        min_val[2] = ascii_float[2];
                    }
                }
            }
        } else {
            new_min_val_is_neg = true;
            min_val[0] = ascii_float[0];
            min_val[1] = ascii_float[1];
            min_val[2] = ascii_float[2];
        }
    } else {
        if !min_is_neg {
            if min_val[0] > ascii_float[0] {
                min_val[0] = ascii_float[0];
                min_val[1] = ascii_float[1];
                min_val[2] = ascii_float[2];
            } else if min_val[0] == ascii_float[0] {
                if min_val[1] > ascii_float[1] {
                    min_val[1] = ascii_float[1];
                    min_val[2] = ascii_float[2];
                } else if min_val[1] == ascii_float[1] && min_val[2] > ascii_float[2] {
                    min_val[2] = ascii_float[2];
                }
            }
        }
    }

    new_min_val_is_neg
}

#[inline(always)]
pub fn replace_max_if_2_ascii_float_greater_than(
    max_val: &mut [u8; 3],
    max_is_neg: bool,
    ascii_float: [u8; 2],
    ascii_is_neg: bool,
) -> bool {
    let mut new_max_val_is_neg = max_is_neg;

    if ascii_is_neg {
        if max_is_neg {
            if max_val[0] == ZERO_AS_BYTE {
                if max_val[1] > ascii_float[0] {
                    max_val[1] = ascii_float[0];
                    max_val[2] = ascii_float[1];
                } else if max_val[1] == ascii_float[0] && max_val[2] > ascii_float[1] {
                    max_val[2] = ascii_float[1];
                }
            }
        }
    } else {
        if max_is_neg {
            new_max_val_is_neg = false;
            max_val[0] = ZERO_AS_BYTE;
            max_val[1] = ascii_float[0];
            max_val[2] = ascii_float[1];
        } else {
            if max_val[0] == ZERO_AS_BYTE {
                if max_val[1] < ascii_float[0] {
                    max_val[1] = ascii_float[0];
                    max_val[2] = ascii_float[1];
                } else if max_val[1] == ascii_float[0] && max_val[2] < ascii_float[1] {
                    max_val[2] = ascii_float[1];
                }
            }
        }
    }
    new_max_val_is_neg
}

#[inline(always)]
pub fn replace_max_if_3_ascii_float_greater_than(
    max_val: &mut [u8; 3],
    max_is_neg: bool,
    ascii_float: [u8; 3],
    ascii_is_neg: bool,
) -> bool {
    let mut new_max_val_is_neg = max_is_neg;

    if ascii_is_neg {
        if max_is_neg {
            if max_val[0] > ascii_float[0] {
                max_val[0] = ascii_float[0];
                max_val[1] = ascii_float[1];
                max_val[2] = ascii_float[2];
            } else if max_val[0] == ascii_float[0] {
                if max_val[1] > ascii_float[1] {
                    max_val[1] = ascii_float[1];
                    max_val[2] = ascii_float[2];
                } else if max_val[1] == ascii_float[1] && max_val[2] > ascii_float[2] {
                    max_val[2] = ascii_float[2];
                }
            }
        }
    } else {
        if max_is_neg {
            new_max_val_is_neg = false;
            max_val[0] = ascii_float[0];
            max_val[1] = ascii_float[1];
            max_val[2] = ascii_float[2];
        } else {
            if max_val[0] < ascii_float[0] {
                max_val[0] = ascii_float[0];
                max_val[1] = ascii_float[1];
                max_val[2] = ascii_float[2];
            } else if max_val[0] == ascii_float[0] {
                if max_val[1] < ascii_float[1] {
                    max_val[1] = ascii_float[1];
                    max_val[2] = ascii_float[2];
                } else if max_val[1] == ascii_float[1] && max_val[2] < ascii_float[2] {
                    max_val[2] = ascii_float[2];
                }
            }
        }
    }

    new_max_val_is_neg
}
