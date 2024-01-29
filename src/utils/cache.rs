use std::{
    collections::HashMap,
    error::Error,
    fs::OpenOptions,
    io::{Read, Write},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheData<T> {
    pub data: HashMap<String, T>,
}

pub fn load_cache_file<T>(cache_name: &str) -> Result<CacheData<T>, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let cache_dir = dirs::cache_dir().unwrap();
    let app_cache = cache_dir.join("mic");

    if !app_cache.exists() {
        std::fs::create_dir_all(&app_cache)?;
    }

    let file_path = app_cache.join(cache_name);

    Ok(match file_path.exists() {
        true => {
            let cache_file = OpenOptions::new().read(true).open(file_path)?;
            let reader = std::io::BufReader::new(cache_file);

            let mut json_string = String::new();
            reader.take(u64::MAX as u64).read_to_string(&mut json_string)?;

            from_str(&json_string)?
        },
        false => CacheData { data: HashMap::new() },
    })
}

pub fn save_cache_file<T>(data: &CacheData<T>, cache_name: &str) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    let serialized = serde_json::to_string(data)?;
    let cache_dir = dirs::cache_dir().unwrap();
    let app_cache = cache_dir.join("mic");
    let file_path = app_cache.join(cache_name);
    let mut cache_file = OpenOptions::new().create(true).write(true).open(file_path)?;

    cache_file.write_all(serialized.as_bytes())?;

    Ok(())
}
