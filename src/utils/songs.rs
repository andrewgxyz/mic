use std::{
    collections::hash_map::Entry,
    error::Error,
};

use chrono::Datelike;
use dirs;
use glob::{glob_with, MatchOptions};
use lofty::{AudioFile, ItemKey, Probe, Tag, TaggedFileExt};
use serde::{Deserialize, Serialize};

use super::{
    cache::{load_cache_file, save_cache_file},
    data::string_clean,
    date::parse_string_to_datetime,
    filters::{
        match_lyrics_contain_words,
        equals_same_value, 
        match_decade, 
        match_no_lyrics, 
        match_current_week, 
        contains_list_of_strings
    },
};

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
    pub filename: String,
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
    pub _lyrics: Vec<String>,
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
}

#[derive(Default)]
pub struct SongDataFilter {
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

impl SongDataFilter {
    pub fn filter(&self, songs: Vec<SongData>) -> Vec<SongData> {
        songs.into_iter().filter(|song| self.matches(song)).collect()
    }

    pub fn matches(&self, song: &SongData) -> bool {
        let dt = parse_string_to_datetime(&song.recording_date)
            .expect("Something went wrong with date parsing");

        let mut matches: Vec<bool> = vec![];

        // Matching Genres & Moods
        matches.push(contains_list_of_strings(&self.genre, &song._genre));
        matches.push(contains_list_of_strings(&self.moods, &song._mood));

        // Matching Genres
        matches.push(match_lyrics_contain_words(&self.words, &song._lyrics));

        // Matching by release params
        matches.push(equals_same_value::<i32>(&self.year, &dt.year()));
        matches.push(equals_same_value::<u32>(&self.month, &dt.month()));
        matches.push(equals_same_value::<u32>(&self.day, &dt.day()));
        // Matching by Decade
        matches.push(match_decade(dt, &self.decade));

        // Matching by Track info
        matches.push(equals_same_value::<String>(&self.artist, &song._track_artist));
        matches.push(equals_same_value::<String>(&self.album, &song._album_title));
        matches.push(equals_same_value::<String>(&self.track, &song._track_number));


        // Matching by Current Week
        if self.week {
            matches.push(match_current_week(&song.recording_date))
        }

        if self.instrumental {
            matches.push(match_no_lyrics(&song._lyrics));
        }

        matches.iter().all(|&check| check)
    }
}

const CACHE_FILE_NAME: &str = "songs_cache.json";

pub fn phrases_to_words(phrases: String) -> Vec<String> {
    let conjunctions = [
        "and",
        "but",
        "or",
        "nor",
        "for",
        "so",
        "yet",
        "because",
        "although",
        "though",
        "even though",
        "while",
        "if",
        "unless",
        "until",
        "after",
        "before",
        "since",
        "when",
        "whenever",
        "where",
        "wherever",
        "whether",
        "as",
        "than",
        "that",
        "whether",
        "now",
    ];

    string_clean(phrases)
        .split_whitespace()
        .filter(|w| !conjunctions.contains(w) && w.len() > 4)
        .map(|s| s.trim_matches(|c: char| !c.is_ascii_alphabetic()).to_string())
        .collect()
}

pub fn get_songs() -> Result<Vec<SongData>, Box<dyn Error>> {
    let music_dir = dirs::audio_dir().unwrap();
    let pattern = format!("{}/*/*/*[.flac,.mp3,.wav]", music_dir.to_string_lossy());

    Ok(songs_list(pattern)?)
}

pub fn get_albums() -> Result<Vec<SongData>, Box<dyn Error>> {
    let music_dir = dirs::audio_dir().unwrap();
    let pattern = format!("{}/*/*/*01-*[.flac,.mp3,.wav]", music_dir.to_string_lossy());

    Ok(songs_list(pattern)?)
}

/// Collects a list of songs by pattern
pub fn songs_list(music_pattern: String) -> Result<Vec<SongData>, Box<dyn Error>> {
    let globs = glob_with(
        &music_pattern,
        MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        },
    )?;
    let mut cache_file = load_cache_file::<SongData>(CACHE_FILE_NAME)?;
    let paths: Vec<_> = globs.filter_map(|entry| entry.ok()).collect();

    for path in paths {
        let filename = path.display().to_string();

        match cache_file.data.entry(filename.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(load_song_tag(&filename)),
        };
    }

    save_cache_file::<SongData>(&cache_file, CACHE_FILE_NAME)?;

    let map_to_vec: Vec<SongData> = {
        let cache_file_ref = cache_file;
        cache_file_ref.data.values().cloned().collect()
    };

    Ok(map_to_vec)
}

fn get_tag(tag: &Tag, key: &ItemKey) -> String {
    tag.get_string(key).unwrap_or("None").to_string()
}

pub fn load_song_tag(filename: &String) -> SongData {
    let tagged_file = Probe::open(filename)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let tag = match tagged_file.primary_tag() {
        Some(primary) => primary,
        None => tagged_file.first_tag().expect("ERROR: No tags found!"),
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
    let genre: Vec<String> =
        get_tag(tag, &ItemKey::Genre).split(';').map(|s| s.to_string()).collect();
    let involved_people = get_tag(tag, &ItemKey::InvolvedPeople)
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let label = get_tag(tag, &ItemKey::Label);
    let language = get_tag(tag, &ItemKey::Language);
    let length = get_tag(tag, &ItemKey::Length);
    let license = get_tag(tag, &ItemKey::License);
    let lyricist = get_tag(tag, &ItemKey::Lyricist).split(',').map(|s| s.to_string()).collect();
    let lyrics = get_tag(tag, &ItemKey::Lyrics).split("\n\n").map(|s| s.to_string()).collect();
    let mix_dj = get_tag(tag, &ItemKey::MixDj).split(',').map(|s| s.to_string()).collect();
    let mix_engineer =
        get_tag(tag, &ItemKey::MixEngineer).split(',').map(|s| s.to_string()).collect();
    let mood: Vec<String> =
        get_tag(tag, &ItemKey::Mood).split(',').map(|s| s.to_string()).collect();
    let movement = get_tag(tag, &ItemKey::Movement);
    let movement_number = get_tag(tag, &ItemKey::MovementNumber);
    let movement_total = get_tag(tag, &ItemKey::MovementTotal);
    let musician_credits = get_tag(tag, &ItemKey::MusicianCredits)
        .split(',')
        .map(|s| s.to_string())
        .collect();
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
        filename: filename.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::load_song_tag;

    #[test]
    fn can_load_tag() {
        let tag =
            load_song_tag(&"/home/andrew/music/adam-lambert/velvet/01-velvet.flac".to_string());

        assert_eq!(tag._track_artist, "Adam Lambert");
    }
}
