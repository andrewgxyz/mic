use std::collections::HashMap;

pub fn array_truncate<T>(arr: &mut Vec<T>, len: Option<usize>) {
    match len {
        Some(len) => {
            let new_length = arr.len().min(len);

            arr.truncate(new_length);
        },
        None => ()
    };
}

pub fn hashmap_to_vec_truple<K, V>(data: HashMap<K, V>) -> Vec<(K, V)> {
    data.into_iter()
        .collect::<Vec<(K, V)>>()
}
