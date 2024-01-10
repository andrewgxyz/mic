use std::{
    collections::HashMap, 
    fs::OpenOptions, 
    error::Error, 
    io::{Write, Read}
};

use chrono::Datelike;
use dirs;
use glob::{glob_with, MatchOptions};
use lofty::{Probe, TaggedFileExt, ItemKey, Tag, AudioFile};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use super::date::parse_string_to_datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongData {
    // Track Info
    pub _album_artist: String,
    pub _album_title: String,
    pub _catalog_number: String,
    pub _genre: Vec<String>,
    pub _mood: Vec<String>,
    pub _movement: String,
    pub _movement_number: String,
    pub _movement_total: String,
    pub _track_artist: String,
    pub _track_length: u64,
    pub _track_number: String,
    pub _track_title: String,
    pub _track_total: String,
    pub recording_date: String,
    // Details and Comments
    pub _comment: String,
    pub _copyright_message: String,
    pub _description: String,
    pub _label: String,
    pub _license: String,
    pub _parental_advisory: String,
    // Production Credits
    pub _arranger: Vec<String>,
    pub _composer: Vec<String>,
    pub _conductor: Vec<String>,
    pub _director: Vec<String>,
    pub _engineer: Vec<String>,
    pub _involved_people: Vec<String>,
    pub _language: String,
    pub _length: String,
    pub _lyricist: Vec<String>,
    pub _lyrics: String,
    pub _mix_dj: Vec<String>,
    pub _mix_engineer: Vec<String>,
    pub _musician_credits: String,
    pub _performer: Vec<String>,
    pub _producer: Vec<String>,
    pub _publisher: String,
    pub _remixer: String,
    pub _script: String,
    pub _work: String,
    pub _writer: Vec<String>,
    pub filename: String
}

#[derive(Default)]
pub struct SongDataFilter {
    pub month: Option<u32>,
    pub year: Option<i32>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
}

impl SongDataFilter {
    pub fn filter(&self, songs: Vec<SongData>) -> Vec<SongData> {
        songs
            .into_iter()
            .filter(|song| self.matches(song))
            .collect()
    }

    pub fn matches(&self, song: &SongData) -> bool {
        let dt = parse_string_to_datetime(&song.recording_date).expect("Something went wrong with date parsing");

        let genre_match = match &self.genre {
            Some(genre) => song._genre.contains(genre),
            None => true,
        };

        let year_match = match self.year {
            Some(year) => dt.year() == year,
            None => true,
        };

        let month_match = match self.month {
            Some(month) => dt.month() == month,
            None => true,
        };

        let artist_match = match &self.artist {
            Some(artist) => &song._track_artist == artist,
            None => true,
        };

        let album_match = match &self.album {
            Some(album) => &song._album_title == album,
            None => true,
        };

        year_match && month_match 
            && artist_match && album_match && genre_match 
    }
}

#[derive(Serialize, Deserialize)]
pub struct SongDataCache {
    songs: HashMap<String, SongData>
}

const CACHE_FILE_NAME: &str = "songs_cache.json";

pub fn get_songs() -> Result<Vec<SongData>, Box<dyn Error>> {
    let music_dir = dirs::audio_dir().unwrap();
    let pattern = format!("{}/*/*/*[.flac,.mp3,.wav]", music_dir.to_string_lossy());

    Ok(songs_list(pattern)?)
}

pub fn get_albums() -> Result<Vec<SongData>, Box<dyn Error>> {
    let music_dir = dirs::audio_dir().unwrap();
    let pattern = format!("{}/*/*/01-*[.flac,.mp3,.wav]", music_dir.to_string_lossy());

    Ok(songs_list(pattern)?)
}

fn load_cache_file(cache_name: &str) -> Result<SongDataCache, Box<dyn Error>> {
    let cache_dir = dirs::cache_dir().unwrap();
    let app_cache = cache_dir.join("mic");

    if !app_cache.exists() {
        std::fs::create_dir_all(&app_cache)?;
    }

    let file_path = app_cache.join(cache_name);

    Ok(match file_path.exists() {
        true => {
            let cache_file = OpenOptions::new()
                .read(true)
                .open(file_path)?;
            let reader = std::io::BufReader::new(cache_file);
            println!("Cache has been loaded");

            let mut json_string = String::new();
            reader.take(u64::MAX as u64).read_to_string(&mut json_string)?;
            let cached_song_tags: SongDataCache = from_str(&json_string)?;

            cached_song_tags
        },
        false => {
            SongDataCache {
                songs: HashMap::new()
            }
        }
    })
}

fn save_cache_file (data: &SongDataCache, cache_name: &str) ->Result<(), Box<dyn Error>> {
    let serialized = serde_json::to_string(data)?;
    let cache_dir = dirs::cache_dir().unwrap();
    let app_cache = cache_dir.join("mic");
    let file_path = app_cache.join(cache_name);
    let mut cache_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)?;

    cache_file.write_all(serialized.as_bytes())?;

    Ok(())
}

/// Collects a list of songs by filter
pub fn songs_list(music_pattern: String) -> Result<Vec<SongData>, Box<dyn Error>> {
    let globs = glob_with(&music_pattern, MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    })?;
    let mut cache_file = load_cache_file(CACHE_FILE_NAME)?;

    let mut songs: Vec<SongData> = vec![];

    for entry in globs.flatten() {
        let filename = entry.display().to_string();

        if cache_file.songs.contains_key(&filename) {
            let song = cache_file.songs.get(&filename).unwrap().clone();

            songs.push(song);
            continue;
        }

        let tag = load_song_tag(&filename);
        cache_file.songs.insert(filename, tag.clone());

        songs.push(tag);
    }

    save_cache_file(&cache_file, CACHE_FILE_NAME)?;

    Ok(songs)
}

fn get_tag (tag: &Tag, key: &ItemKey) -> String {
    tag.get_string(key).unwrap_or("None").to_string()
}

fn load_song_tag (filename: &String) -> SongData {
    let tagged_file = Probe::open(filename)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let tag = match tagged_file.primary_tag() {
        Some(primary) => primary,
        None => tagged_file.first_tag().expect("ERROR: No tags found!")
    };

    let album_artist = get_tag(tag, &ItemKey::AlbumArtist);
    let album_title = get_tag(tag, &ItemKey::AlbumTitle);
    let arranger = get_tag(tag, &ItemKey::Arranger).split(',').map(|s| s.to_string()).collect();
    let catalog_number = get_tag(tag, &ItemKey::CatalogNumber);
    let comment = get_tag(tag, &ItemKey::Comment);
    let composer = get_tag(tag, &ItemKey::Composer).split(',').map(|s| s.to_string()).collect();
    let conductor = get_tag(tag, &ItemKey::Conductor).split(',').map(|s| s.to_string()).collect();
    let copyright_message = get_tag(tag, &ItemKey::CopyrightMessage);
    let description = get_tag(tag, &ItemKey::Description);
    let director = get_tag(tag, &ItemKey::Director).split(',').map(|s| s.to_string()).collect();
    let engineer = get_tag(tag, &ItemKey::Engineer).split(',').map(|s| s.to_string()).collect();
    let genre: Vec<String> = get_tag(tag, &ItemKey::Genre).split(';').map(|s| s.to_string()).collect();
    let involved_people = get_tag(tag, &ItemKey::InvolvedPeople).split(',').map(|s| s.to_string()).collect();
    let label = get_tag(tag, &ItemKey::Label);
    let language = get_tag(tag, &ItemKey::Language);
    let length = get_tag(tag, &ItemKey::Length);
    let license = get_tag(tag, &ItemKey::License);
    let lyricist = get_tag(tag, &ItemKey::Lyricist).split(',').map(|s| s.to_string()).collect();
    let lyrics = get_tag(tag, &ItemKey::Lyrics);
    let mix_dj = get_tag(tag, &ItemKey::MixDj).split(',').map(|s| s.to_string()).collect();
    let mix_engineer = get_tag(tag, &ItemKey::MixEngineer).split(',').map(|s| s.to_string()).collect();
    let mood: Vec<String>= get_tag(tag, &ItemKey::Mood).split(',').map(|s| s.to_string()).collect();
    let movement = get_tag(tag, &ItemKey::Movement);
    let movement_number = get_tag(tag, &ItemKey::MovementNumber);
    let movement_total = get_tag(tag, &ItemKey::MovementTotal);
    let musician_credits = get_tag(tag, &ItemKey::MusicianCredits).split(',').map(|s| s.to_string()).collect();
    let parental_advisory = get_tag(tag, &ItemKey::ParentalAdvisory);
    let performer = get_tag(tag, &ItemKey::Performer).split(',').map(|s| s.to_string()).collect();
    let producer = get_tag(tag, &ItemKey::Producer).split(',').map(|s| s.to_string()).collect();
    let publisher = get_tag(tag, &ItemKey::Publisher);
    let recording_date = get_tag(tag, &ItemKey::RecordingDate);
    let remixer = get_tag(tag, &ItemKey::Remixer);
    let script = get_tag(tag, &ItemKey::Script);
    let track_length = tagged_file.properties().duration().as_secs();
    let track_artist = get_tag(tag, &ItemKey::TrackArtist);
    let track_number = get_tag(tag, &ItemKey::TrackNumber);
    let track_title = get_tag(tag, &ItemKey::TrackTitle);
    let track_total = get_tag(tag, &ItemKey::TrackTotal);
    let work = get_tag(tag, &ItemKey::Work);
    let writer = get_tag(tag, &ItemKey::Writer).split(',').map(|s| s.to_string()).collect();

    SongData {
        _album_artist: album_artist,
        _album_title: album_title,
        _arranger: arranger,
        _catalog_number: catalog_number,
        _comment: comment,
        _composer: composer,
        _conductor: conductor,
        _copyright_message: copyright_message,
        _description: description,
        _director: director,
        _engineer: engineer,
        _genre: genre,
        _involved_people: involved_people,
        _label: label,
        _language: language,
        _length: length,
        _license: license,
        _lyricist: lyricist,
        _lyrics: lyrics,
        _mix_dj: mix_dj,
        _mix_engineer: mix_engineer,
        _mood: mood,
        _movement: movement,
        _movement_number: movement_number,
        _movement_total: movement_total,
        _musician_credits: musician_credits,
        _parental_advisory: parental_advisory,
        _performer: performer,
        _producer: producer,
        _publisher: publisher,
        recording_date,
        _remixer: remixer,
        _script: script,
        _track_length: track_length,
        _track_artist: track_artist,
        _track_number: track_number,
        _track_title: track_title,
        _track_total: track_total,
        _work: work,
        _writer: writer,
        filename: filename.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::load_song_tag;

    #[test]
    fn can_load_tag () {
        let tag = load_song_tag(&"/home/andrew/music/adam-lambert/velvet/01-velvet.flac".to_string());

        assert_eq!(tag._track_artist, "Adam Lambert");
    }
}
