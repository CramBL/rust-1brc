#![allow(unused_imports, dead_code)]

use crate::util::*;

#[inline(always)]
fn configured_mmap_read() -> Mmap {
    use memmap2::Advice;
    let file = unsafe { File::open(FILE_PATH).unwrap_unchecked() };

    let mmap = unsafe { Mmap::map(&file).unwrap_unchecked() };
    unsafe { mmap.advise(Advice::PopulateRead).unwrap_unchecked() };

    mmap
}

#[inline(always)]
fn divide_mmap_regions(mmap: &Mmap) -> Vec<(usize, usize)> {
    const REGION_SIZE: usize = 1024 * 1024 * 500;
    let (mut start, mut end) = (0, REGION_SIZE);
    let mut regions = Vec::with_capacity(20);

    let end_mmap = mmap.len();
    while end < end_mmap - 1 {
        end = if end + REGION_SIZE >= end_mmap {
            let end = end_mmap - 1;
            regions.push((start, end));
            break;
        } else {
            end + REGION_SIZE
        };

        let next_newline_pos = mmap.iter().skip(end).position(|&x| x == NEWLINE_BYTE);
        if let Some(next_newline_pos) = next_newline_pos {
            end += next_newline_pos;
        }

        regions.push((start, end));
        start = end + 1;
    }

    regions
}

fn region_worker(region: &[u8]) -> Vec<Vec<([u8; 101], Vec<[u8; 6]>)>> {
    let mut city_start_letters: Vec<u8> = Vec::with_capacity(500);

    let mut city_temps: Vec<Vec<([u8; 101], Vec<[u8; 6]>)>> = Vec::with_capacity(500);

    // Create an array that holds both the city name and the temperature array
    let mut name_temps_arr: [[u8; 100]; 2] = [[0; 100]; 2];

    let mut is_temp = false;
    let mut end_name_idx = 0;
    let mut end_temp_idx;

    let mut insert_idx = 0;

    for byte in region.iter() {
        if *byte == SEMI_COLON_BYTE {
            is_temp = true;
            end_name_idx = insert_idx - 1;
            insert_idx = 0;
        } else if *byte == NEWLINE_BYTE {
            is_temp = false;
            end_temp_idx = insert_idx - 1;
            insert_idx = 0;

            // Insert the name and temps into the collection
            let mut city_name: [u8; 101] = [0; 101];
            unsafe { *city_name.get_unchecked_mut(100) = end_name_idx as u8 };
            for i in 0..=end_name_idx {
                unsafe { *city_name.get_unchecked_mut(i) = name_temps_arr[0][i] };
            }

            let mut temps: [u8; 6] = [0, 0, 0, 0, 0, end_temp_idx as u8];
            for i in 0..=end_temp_idx {
                temps[i] = name_temps_arr[1][i];
            }

            let city_temps_idx = city_start_letters.iter().position(|&x| x == city_name[0]);
            if let Some(city_temps_idx) = city_temps_idx {
                let existing_pos = city_temps[city_temps_idx]
                    .iter()
                    .position(|(name, _)| name == &city_name);
                if let Some(existing_pos) = existing_pos {
                    city_temps[city_temps_idx][existing_pos].1.push(temps);
                } else {
                    let mut temps_vec = Vec::with_capacity(100);
                    temps_vec.push(temps);
                    city_temps[city_temps_idx].push((city_name, temps_vec));
                }
            } else {
                city_start_letters.push(city_name[0]);
                let mut temps_vec = Vec::with_capacity(100);
                temps_vec.push(temps);
                city_temps.push(vec![(city_name, temps_vec)]);
            }
        } else {
            name_temps_arr[is_temp as usize][insert_idx] = *byte;
            insert_idx += 1;
        }
    }

    city_temps
}

pub fn do_work() {
    println!("== Starting parallel processing ==");
    let total_time = std::time::Instant::now();
    let step_time = std::time::Instant::now();

    let contents = configured_mmap_read();

    let step_time = print_progress_timing("mmap configured done", step_time, total_time);

    let regions = divide_mmap_regions(&contents);
    let mut step_time = print_progress_timing("regions divided", step_time, total_time);

    let mut city_start_letters: Vec<u8> = Vec::with_capacity(500);

    let mut city_temps: Vec<Vec<([u8; 101], Vec<[u8; 6]>)>> = Vec::with_capacity(500);

    for region in regions {
        let (start, end) = region;

        let contents = &contents[start..end];

        let city_temps_region = region_worker(contents);
        step_time = print_progress_timing("processed region", step_time, total_time);

        city_temps_region
            .into_iter()
            .for_each(|city_start_letter_vec| {
                // Find if the city start letter is already in the city_start_letters
                let city_start_letter = city_start_letter_vec[0].0[0];
                let city_temps_idx = city_start_letters
                    .iter()
                    .position(|&x| x == city_start_letter);
                if let Some(city_temps_idx) = city_temps_idx {
                    city_start_letter_vec.into_iter().for_each(|(name, temps)| {
                        let existing_pos = city_temps[city_temps_idx]
                            .iter()
                            .position(|(name_, _)| name_ == &name);
                        if let Some(existing_pos) = existing_pos {
                            city_temps[city_temps_idx][existing_pos].1.extend(temps);
                        } else {
                            city_temps[city_temps_idx].push((name, temps));
                        }
                    });
                } else {
                    city_start_letters.push(city_start_letter);
                    city_temps.push(city_start_letter_vec);
                }
            });
        step_time = print_progress_timing("Aggregated region data", step_time, total_time);
    }
    let processing_data_time = step_time.elapsed();
    let step_time = print_progress_timing("processed all data", step_time, total_time);

    // Print the map alphabetically ordered like `{Abha=5.0/18.0/27.4, Abidjan=15.7/26.0/34.1, ...}`

    // How many cities are there?
    let mut city_count: u16 = 0;
    for city in city_temps.iter() {
        city_count += city.len() as u16;
    }
    println!("Number of cities: {city_count}");

    // Sort the city names alphabetically
    city_temps.sort_by(|a, b| a[0].0.cmp(&b[0].0));

    let results = calc_temps(city_temps);
    let post_processing_time = step_time.elapsed();
    let step_time = print_progress_timing("Post processing city temps done", step_time, total_time);

    final_print(results);

    let _ = print_progress_timing("final print done", step_time, total_time);

    println!("==\n== total time elapsed: {:?} ==", total_time.elapsed());
    println!("--> Processing data: {processing_data_time:?}");
    println!("--> Post processing: {post_processing_time:?}");
}

fn final_print(results: Vec<(Box<str>, f32, f32, f32)>) {
    let mut remaining_cnt = results.len();

    print!("{{");
    for (name, min, mean, max) in results {
        print!("{name}=");

        print!("{min:.1}/{mean:.1}/{max:.1}");

        if remaining_cnt > 1 {
            print!(", ");
        }
        remaining_cnt -= 1;
    }
    println!("}}");
}

#[inline(always)]
pub fn calc_temps(
    city_temps: Vec<Vec<([u8; 101], Vec<[u8; 6]>)>>,
) -> Vec<(Box<str>, f32, f32, f32)> {
    let mut final_city_temps: Vec<(Box<str>, f32, f32, f32)> = Vec::new();

    for name_temp_vec in city_temps {
        let fct = calc_city_temp_vec(name_temp_vec);
        final_city_temps.extend(fct);
    }

    final_city_temps
}

#[inline(always)]
pub fn calc_city_temp_vec(
    mut city_temps: Vec<([u8; 101], Vec<[u8; 6]>)>,
) -> Vec<(Box<str>, f32, f32, f32)> {
    // first sort it alphabetically
    city_temps.sort_by(|a, b| a.0.cmp(&b.0));

    let mut final_city_temps: Vec<(Box<str>, f32, f32, f32)> = Vec::with_capacity(city_temps.len());

    for (name, temps) in city_temps {
        let (min_temp, mean_temp, max_temp) = calc_min_mean_max_temps_alt(temps);
        let name = unsafe {
            let name_slice = &name[0..=*name.get_unchecked(100) as usize];
            String::from_utf8_unchecked(name_slice.to_vec())
        };
        final_city_temps.push((name.into_boxed_str(), min_temp, mean_temp, max_temp));
    }

    final_city_temps
}

#[inline(always)]
pub fn calc_min_mean_max_temps_alt(temps: Vec<[u8; 6]>) -> (f32, f32, f32) {
    let total_elements = temps.len();
    let mut min_temp = f32::MAX;
    let mut max_temp = f32::MIN;
    let mut sum_temp = 0.0;

    for temp in temps {
        let end_temp_idx = temp[5] as usize;
        let temp = unsafe {
            String::from_utf8_unchecked((&temp[0..=end_temp_idx]).to_vec())
                .parse::<f32>()
                .unwrap_unchecked()
        };

        sum_temp += temp;
        if temp < min_temp {
            min_temp = temp;
        }
        if temp > max_temp {
            max_temp = temp;
        }
    }

    let mean_temp = sum_temp / total_elements as f32;
    (min_temp, mean_temp, max_temp)
}
