use std::collections::HashMap;

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
