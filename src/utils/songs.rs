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
    data::{string_clean, string_to_vec},
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
    pub album_artist: String,
    pub album_title: String,
    pub catalog_number: String,
    pub genre: Vec<String>,
    pub length: String,
    pub lyrics: Vec<String>,
    pub mood: Vec<String>,
    pub movement: String,
    pub movement_number: String,
    pub movement_total: String,
    pub track_artist: String,
    pub track_length: u64,
    pub track_number: String,
    pub track_title: String,
    pub track_total: String,
    pub filename: String,
    pub recording_date: String,

    // Details and Comments
    pub comment: String,
    pub copyright_message: String,
    pub description: String,
    pub label: String,
    pub language: String,
    pub license: String,
    pub parental_advisory: String,

    // Production Credits
    pub arranger: Vec<String>,
    pub composer: Vec<String>,
    pub conductor: Vec<String>,
    pub director: Vec<String>,
    pub engineer: Vec<String>,
    pub involved_people: Vec<String>,
    pub lyricist: Vec<String>,
    pub mix_dj: Vec<String>,
    pub mix_engineer: Vec<String>,
    pub musician_credits: Vec<String>,
    pub performer: Vec<String>,
    pub producer: Vec<String>,
    pub publisher: String,
    pub remixer: String,
    pub work: String,
    pub writer: Vec<String>,
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
        matches.push(contains_list_of_strings(&self.genre, &song.genre));
        matches.push(contains_list_of_strings(&self.moods, &song.mood));

        // Matching Genres
        println!("{}", song.filename);
        matches.push(match_lyrics_contain_words(&self.words, &song.lyrics));

        // Matching by release params
        matches.push(equals_same_value::<i32>(&self.year, &dt.year()));
        matches.push(equals_same_value::<u32>(&self.month, &dt.month()));
        matches.push(equals_same_value::<u32>(&self.day, &dt.day()));
        // Matching by Decade
        matches.push(match_decade(dt, &self.decade));

        // Matching by Track info
        matches.push(equals_same_value::<String>(&self.artist, &song.track_artist));
        matches.push(equals_same_value::<String>(&self.album, &song.album_title));
        matches.push(equals_same_value::<String>(&self.track, &song.track_number));


        // Matching by Current Week
        if self.week {
            matches.push(match_current_week(&song.recording_date))
        }

        if self.instrumental {
            matches.push(match_no_lyrics(&song.lyrics));
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
    match tag.get_string(key) {
        Some(value) => value,
        None => ""
    }.to_string()
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
    let arranger = string_to_vec(get_tag(tag, &ItemKey::Arranger), ",");
    let catalog_number = get_tag(tag, &ItemKey::CatalogNumber);
    let comment = get_tag(tag, &ItemKey::Comment);
    let composer = string_to_vec(get_tag(tag, &ItemKey::Composer), ",");
    let conductor = string_to_vec(get_tag(tag, &ItemKey::Conductor), ",");
    let copyright_message = get_tag(tag, &ItemKey::CopyrightMessage);
    let description = get_tag(tag, &ItemKey::Description);
    let director = string_to_vec(get_tag(tag, &ItemKey::Director), ",");
    let engineer = string_to_vec(get_tag(tag, &ItemKey::Engineer), ",");
    let genre: Vec<String> = string_to_vec(get_tag(tag, &ItemKey::Genre), ";");
    let involved_people = string_to_vec(get_tag(tag, &ItemKey::InvolvedPeople), ",");
    let label = get_tag(tag, &ItemKey::Label);
    let language = get_tag(tag, &ItemKey::Language);
    let length = get_tag(tag, &ItemKey::Length);
    let license = get_tag(tag, &ItemKey::License);
    let lyricist = string_to_vec(get_tag(tag, &ItemKey::Lyricist), ",");
    let lyrics = string_to_vec(get_tag(tag, &ItemKey::Lyrics), "\n\n");
    let mix_dj = string_to_vec(get_tag(tag, &ItemKey::MixDj), ",");
    let mix_engineer = string_to_vec(get_tag(tag, &ItemKey::MixEngineer), ",");
    let mood = string_to_vec(get_tag(tag, &ItemKey::Mood), ",");
    let movement = get_tag(tag, &ItemKey::Movement);
    let movement_number = get_tag(tag, &ItemKey::MovementNumber);
    let movement_total = get_tag(tag, &ItemKey::MovementTotal);
    let musician_credits = string_to_vec(get_tag(tag, &ItemKey::MusicianCredits), ",");
    let parental_advisory = get_tag(tag, &ItemKey::ParentalAdvisory);
    let performer = string_to_vec(get_tag(tag, &ItemKey::Performer), ",");
    let producer = string_to_vec(get_tag(tag, &ItemKey::Producer), ",");
    let publisher = get_tag(tag, &ItemKey::Publisher);
    let recording_date = get_tag(tag, &ItemKey::RecordingDate);
    let remixer = get_tag(tag, &ItemKey::Remixer);
    let track_length = tagged_file.properties().duration().as_secs();
    let track_artist = get_tag(tag, &ItemKey::TrackArtist);
    let track_number = get_tag(tag, &ItemKey::TrackNumber);
    let track_title = get_tag(tag, &ItemKey::TrackTitle);
    let track_total = get_tag(tag, &ItemKey::TrackTotal);
    let work = get_tag(tag, &ItemKey::Work);
    let writer = string_to_vec(get_tag(tag, &ItemKey::Writer), ",");

    SongData {
        album_artist,
        album_title,
        arranger,
        catalog_number,
        comment,
        composer,
        conductor,
        copyright_message,
        description,
        director,
        engineer,
        genre,
        involved_people,
        label,
        language,
        length,
        license,
        lyricist,
        lyrics,
        mix_dj,
        mix_engineer,
        mood,
        movement,
        movement_number,
        movement_total,
        musician_credits,
        parental_advisory,
        performer,
        producer,
        publisher,
        recording_date,
        remixer,
        track_length,
        track_artist,
        track_number,
        track_title,
        track_total,
        work,
        writer,
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

        assert_eq!(tag.track_artist, "Adam Lambert");
    }
}
