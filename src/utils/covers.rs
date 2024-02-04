use std::{
    collections::hash_map::Entry,
    error::Error,
    sync::{Arc, Mutex},
};

use chrono::Datelike;
use glob::{glob_with, MatchOptions};
use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};

use super::{
    cache::{load_cache_file, save_cache_file},
    date::parse_string_to_datetime,
    songs::{get_albums, SongData},
    filters::{contains_list_of_strings, equals_same_value, match_decade, match_current_week, match_no_lyrics, match_lyrics_contain_words}, data::vec_u8_to_vec_point,
    kmeans::k_means
};

use rayon::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumCoverData {
    pub image: ImageCache,
    pub album_data: SongData,
    pub cover_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageCache {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub dominant_colors: Vec<(u8, u8, u8)>
}

#[derive(Default)]
pub struct AlbumCoverDataFilter {
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<i32>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub moods: Option<String>,
    pub words: Option<String>,
    pub decade: Option<u16>,
    pub track: Option<String>,
    pub week: bool,
    pub instrumental: bool,
}

impl AlbumCoverDataFilter {
    pub fn filter(&self, covers: Vec<AlbumCoverData>) -> Vec<AlbumCoverData> {
        covers.into_iter().filter(|cover| self.matches(cover)).collect()
    }

    pub fn matches(&self, cover: &AlbumCoverData) -> bool {
        let dt = parse_string_to_datetime(&cover.album_data.recording_date)
            .expect("Something went wrong with date parsing");

        let mut matches: Vec<bool> = vec![];

        // Matching Genres & Moods
        matches.push(contains_list_of_strings(&self.genre, &cover.album_data._genre));
        matches.push(contains_list_of_strings(&self.moods, &cover.album_data._mood));

        // Matching Genres
        matches.push(match_lyrics_contain_words(&self.words, &cover.album_data._lyrics));

        // Matching by release params
        matches.push(equals_same_value::<i32>(&self.year, &dt.year()));
        matches.push(equals_same_value::<u32>(&self.month, &dt.month()));
        matches.push(equals_same_value::<u32>(&self.day, &dt.day()));
        // Matching by Decade
        matches.push(match_decade(dt, &self.decade));

        // Matching by Track info
        matches.push(equals_same_value::<String>(&self.artist, &cover.album_data._track_artist));
        matches.push(equals_same_value::<String>(&self.album, &cover.album_data._album_title));
        matches.push(equals_same_value::<String>(&self.track, &cover.album_data._track_number));


        // Matching by Current Week
        if self.week {
            matches.push(match_current_week(&cover.album_data.recording_date))
        }

        if self.instrumental && cover.album_data._lyrics.len() == 0 {
            matches.push(match_no_lyrics(&cover.album_data._lyrics));
        }

        matches.iter().all(|&check| check)
    }
}

impl From<DynamicImage> for ImageCache {
    fn from(img: DynamicImage) -> Self {
        let resize = img.resize_exact(480, 480, image::imageops::FilterType::Triangle);
        let (width, height) = resize.dimensions();
        let pixels = resize.to_rgb8().into_raw();
        let points = vec_u8_to_vec_point(pixels.clone(), width);
        let clusters = k_means(points, 5, 10);
        let dominant_colors: Vec<(u8,u8,u8)> = clusters.iter().map(|cluster| cluster.centroid.color).collect();

        ImageCache { width, height, pixels, dominant_colors }
    }
}
const CACHE_FILE_NAME: &str = "cover_cache.json";

pub fn get_album_covers() -> Result<Vec<AlbumCoverData>, Box<dyn Error>> {
    let music_dir = dirs::audio_dir().unwrap();
    let pattern = format!("{}/*/*/cover.*", music_dir.to_string_lossy());

    Ok(load_covers(pattern)?)
}

/// Collects a list of songs by filter
fn load_covers(pattern: String) -> Result<Vec<AlbumCoverData>, Box<dyn Error>> {
    // rayon::ThreadPoolBuilder::new().num_threads(4).build_global()?;
    let globs = glob_with(
        &pattern,
        MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        },
    )?;
    let cache_file = Arc::new(Mutex::new(load_cache_file::<AlbumCoverData>(CACHE_FILE_NAME)?));
    let songs = get_albums()?;

    let file_paths: Vec<_> = globs.filter_map(|entry| entry.ok()).collect();

    file_paths.par_iter().for_each(|path| {
        let filename = path.display().to_string();
        let cache_file_ref = Arc::clone(&cache_file);
        let mut cache_file_ref = cache_file_ref.lock().unwrap();

        match cache_file_ref.data.entry(filename.clone()) {
            Entry::Occupied(o) => {
                // println!("Cached: {}", filename);
                o.into_mut()
            },
            Entry::Vacant(v) => {
                let img = image::open(filename.clone()).unwrap();

                let mut arr_file_dir = filename.split('/')
                    .collect::<Vec<&str>>();
                arr_file_dir.pop();
                let dir = arr_file_dir.join("/");

                // println!("New: {}", dir);
                let song: Vec<_> = songs.iter()
                    .filter(|s| s.filename
                    .contains(&dir)).collect();

                let ic: ImageCache = ImageCache::from(img);

                v.insert(AlbumCoverData {
                    image: ic,
                    album_data: song[0].clone(),
                    cover_name: filename,
                })
            },
        };
    });

    save_cache_file::<AlbumCoverData>(&cache_file.lock().unwrap(), CACHE_FILE_NAME)?;

    let map_to_vec: Vec<AlbumCoverData> = {
        let cache_file_ref = cache_file.lock().unwrap();
        cache_file_ref.data.values().cloned().collect()
    };

    Ok(map_to_vec)
}
