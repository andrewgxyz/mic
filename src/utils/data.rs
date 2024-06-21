use std::collections::HashMap;
use std::error::Error;

use regex::Regex;

use super::kmeans::Point;

pub fn array_truncate<T>(arr: &mut Vec<T>, len: Option<usize>) {
    match len {
        Some(len) => {
            let new_length = arr.len().min(len);

            arr.truncate(new_length);
        },
        None => (),
    };
}

pub fn hashmap_to_vec_truple<K, V>(data: HashMap<K, V>) -> Vec<(K, V)> {
    data.into_iter().collect::<Vec<(K, V)>>()
}

pub fn string_clean(str: String) -> String {
    str.to_lowercase()
}

pub fn string_to_vec(data: String, pattern: &str) -> Vec<String> {
    data.split(pattern).filter(|d| !d.is_empty()).map(|d| d.to_string()).collect()
}

pub fn vec_u8_to_vec_point(pixels: Vec<u8>, width: u32) -> Vec<Point> {
    pixels.chunks_exact(3)
        .enumerate()
        .map(|(i, chunk)| {
            let x = (i % width as usize) as u8;
            let y = (i / width as usize) as u8;

            let color = (chunk[0], chunk[2], chunk[2]);

            Point { x, y, color }
        })
        .collect()
}

pub fn validate_img_filename(name: &str) -> Result<bool, Box<dyn Error>> {
    let file_extensions = vec!["png", "PNG", "jpeg", "JPG", "JPEG", "jpg"];
    let regex_str = format!("\\.({})$", file_extensions.join("|"));
    let regex = Regex::new(&regex_str)?;

    Ok(regex.is_match(name))
}

pub fn sum_rgb (color: &(u8, u8, u8)) -> u32 {
    color.0 as u32 + color.1 as u32 + color.2 as u32
}

pub fn convert_sec_to_fmt_time(sec: u64) -> String {
    let hrs: i32 = (sec / 3600) as i32;
    let mut min: i32 = (sec / 60) as i32;

    if hrs > 0 {
        min = (sec as i32 - (hrs * 3600)) / 60;
        if hrs > 9 {
            return format!("{:03.0}:{:02.0}:{:02.0}", hrs, min, sec % 60);
        }

        return format!("{:02.0}:{:02.0}:{:02.0}", hrs, min, sec % 60);
    }

    format!("{:02.0}:{:02.0}", min, sec % 60)
}
