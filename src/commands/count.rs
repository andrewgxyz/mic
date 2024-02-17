use chrono::Datelike;
use clap::{Args, Subcommand};
use core::fmt;
use prettytable::{format, row, Row, Table};
use std::{collections::HashMap, error::Error};

use crate::utils::{
    data::{array_truncate, hashmap_to_vec_truple},
    date::parse_string_to_datetime,
    songs::{get_albums, get_songs, phrases_to_words, SongData, SongDataFilter},
};

#[derive(Args)]
pub struct CountArgs {
    /// Filter by Day of release
    #[clap(short = 'd', long = "day")]
    day: Option<u16>,

    /// Filter by Decade of release
    #[clap(short = 'D', long = "decade")]
    decade: Option<u16>,

    /// Filter by Month of release
    #[clap(short = 'm', long = "month")]
    month: Option<u32>,

    /// Filter by Year of release
    #[clap(short = 'y', long = "year")]
    year: Option<i32>,

    /// Mood names ex. "eclectic,warm,dark"
    #[clap(short = 'M', long = "moods")]
    moods: Option<String>,

    /// Apply flag to collect by albums rather than songs
    #[clap(short = 'a', long = "album")]
    album: bool,

    /// Filter by Year of release
    #[clap(subcommand)]
    commands: Option<CountCommands>,
}

#[derive(Subcommand)]
enum CountCommands {
    /// What records to play based on release ranges
    Years(YearsArgs),

    /// What records to play based on release ranges
    Genres(GenreArgs),

    /// What records to play based on release ranges
    Moods(MoodArgs),

    /// What records to play based on release ranges
    Words(WordsArgs),
}

pub fn count_music(args: CountArgs) -> Result<(), Box<dyn Error>> {
    match args.commands {
        Some(CountCommands::Years(args)) => count_years(args)?,
        Some(CountCommands::Genres(args)) => count_genres(args)?,
        Some(CountCommands::Moods(args)) => count_moods(args)?,
        Some(CountCommands::Words(args)) => count_words(args)?,
        _ => count_general(args)?,
    };

    Ok(())
}

pub fn count_general(args: CountArgs) -> Result<(), Box<dyn Error>> {
    let songs: Vec<SongData> = get_songs_or_albums(
        args.album,
        SongDataFilter {
            year: args.year,
            month: args.month,
            moods: args.moods,
            ..Default::default()
        },
    );

    let headers = row!["Name", "Total"];
    let mut table_rows: Vec<(&str, usize)> = vec![];

    let mut albums: Vec<String> = vec![];
    let mut artists: Vec<String> = vec![];
    let mut genres: Vec<String> = vec![];
    let mut moods: Vec<String> = vec![];

    for song in songs.clone() {
        if !albums.contains(&song.album_title) {
            albums.push(song.album_title);
        }

        if !artists.contains(&song.track_artist) {
            artists.push(song.track_artist);
        }

        for genre in song.genre {
            if !genres.contains(&genre) {
                genres.push(genre);
            }
        }

        for mood in song.mood {
            if !moods.contains(&mood) {
                moods.push(mood);
            }
        }
    }

    if !args.album {
        table_rows.push(("Songs", songs.len()));
    }
    table_rows.push(("Albums", albums.len()));
    table_rows.push(("Artists", artists.len()));
    table_rows.push(("Genres", genres.len()));
    table_rows.push(("Moods", moods.len()));

    print_table::<&str, usize>(headers, table_rows);

    Ok(())
}

#[derive(Args)]
pub struct YearsArgs {
    /// Filter by Genre(s) (separate by comma)
    #[clap(short = 'g', long = "genre")]
    genre: Option<String>,

    /// Filter by Day of release
    #[clap(short = 'd', long = "day")]
    day: Option<u16>,

    /// Filter by Decade of release
    #[clap(short = 'D', long = "decade")]
    decade: Option<u16>,

    /// Filter by Month of release
    #[clap(short = 'm', long = "month")]
    month: Option<u32>,

    /// Filter by Year of release
    #[clap(short = 'y', long = "year")]
    year: Option<i32>,

    /// Apply flag to collect by albums rather than songs
    #[clap(short = 'a', long = "album")]
    album: bool,
}

pub fn count_years(args: YearsArgs) -> Result<(), Box<dyn Error>> {
    let headers = get_count_headers("Years", args.album);
    let songs: Vec<SongData> = get_songs_or_albums(
        args.album,
        SongDataFilter {
            genre: args.genre,
            decade: args.decade,
            month: args.month,
            ..Default::default()
        },
    );

    let mut years_hash: HashMap<i32, usize> = HashMap::new();

    for song in songs.iter() {
        let dt = parse_string_to_datetime(&song.recording_date).unwrap();
        let count = years_hash.entry(dt.year()).or_insert(0);

        *count += 1;
    }

    let mut vec_years = hashmap_to_vec_truple::<i32, usize>(years_hash);

    vec_years.sort();
    print_table::<i32, usize>(headers, vec_years);

    Ok(())
}

#[derive(Args)]
pub struct GenreArgs {
    /// Filter by Day of release
    #[clap(short = 'd', long = "day")]
    day: Option<u16>,

    /// Filter by Decade of release
    #[clap(short = 'D', long = "decade")]
    decade: Option<u16>,

    /// Filter by Month of release
    #[clap(short = 'm', long = "month")]
    month: Option<u32>,

    /// Filter by Year of release
    #[clap(short = 'y', long = "year")]
    year: Option<i32>,

    /// Apply flag to collect by albums rather than songs
    #[clap(short = 'a', long = "album")]
    album: bool,
}

pub fn count_genres(args: GenreArgs) -> Result<(), Box<dyn Error>> {
    let headers = get_count_headers("Genres", args.album);
    let songs: Vec<SongData> = get_songs_or_albums(
        args.album,
        SongDataFilter {
            year: args.year,
            month: args.month,
            decade: args.decade,
            ..Default::default()
        },
    );

    let mut genre_hash: HashMap<String, i32> = HashMap::new();

    for song in songs.clone() {
        for genre in song.genre {
            let count = genre_hash.entry(genre).or_insert(0);

            *count += 1;
        }
    }

    let mut vec_genres = hashmap_to_vec_truple::<String, i32>(genre_hash);

    vec_genres.sort_by(|(a_key, a_val), (b_key, b_val)| {
        b_val.cmp(&a_val).then_with(|| a_key.cmp(&b_key))
    });
    print_table::<String, i32>(headers, vec_genres);

    Ok(())
}

#[derive(Args)]
pub struct WordsArgs {
    /// Filter by Day of release
    #[clap(short = 'd', long = "day")]
    day: Option<u16>,

    /// Filter by Decade of release
    #[clap(short = 'D', long = "decade")]
    decade: Option<u16>,

    /// Filter by Month of release
    #[clap(short = 'm', long = "month")]
    month: Option<u32>,

    /// Filter by Year of release
    #[clap(short = 'y', long = "year")]
    year: Option<i32>,

    /// Apply flag to collect by albums rather than songs
    #[clap(short = 'a', long = "album")]
    album: bool,

    /// Return on top numbers
    #[clap(short = 'l', long = "length")]
    length: Option<usize>,
}

pub fn count_words(args: WordsArgs) -> Result<(), Box<dyn Error>> {
    let headers = get_count_headers("Moods", args.album);
    let songs: Vec<SongData> = get_songs_or_albums(
        args.album,
        SongDataFilter {
            year: args.year,
            month: args.month,
            ..Default::default()
        },
    );
    let mut words_hash: HashMap<String, usize> = HashMap::new();

    for song in songs.clone() {
        for phrases in song.lyrics {
            let cleaned = phrases_to_words(phrases);

            for word in cleaned {
                words_hash.entry(word).and_modify(|count| *count += 1).or_insert(1);
            }
        }
    }

    let mut vec_moods = hashmap_to_vec_truple::<String, usize>(words_hash);

    vec_moods.sort_by(|(a_key, a_val), (b_key, b_val)| {
        b_val.cmp(&a_val).then_with(|| a_key.cmp(&b_key))
    });

    array_truncate(&mut vec_moods, args.length);

    print_table::<String, usize>(headers, vec_moods.clone());

    Ok(())
}

#[derive(Args)]
pub struct MoodArgs {
    /// Filter by Day of release
    #[clap(short = 'd', long = "day")]
    day: Option<u16>,

    /// Filter by Decade of release
    #[clap(short = 'D', long = "decade")]
    decade: Option<u16>,

    /// Filter by Month of release
    #[clap(short = 'm', long = "month")]
    month: Option<u32>,

    /// Filter by Year of release
    #[clap(short = 'y', long = "year")]
    year: Option<i32>,

    /// Apply flag to collect by albums rather than songs
    #[clap(short = 'a', long = "album")]
    album: bool,

    /// Return on top numbers
    #[clap(short = 'l', long = "length")]
    length: Option<usize>,
}

pub fn count_moods(args: MoodArgs) -> Result<(), Box<dyn Error>> {
    let headers = get_count_headers("Moods", args.album);
    let songs: Vec<SongData> = get_songs_or_albums(
        args.album,
        SongDataFilter {
            year: args.year,
            month: args.month,
            ..Default::default()
        },
    );
    let mut mood_hash: HashMap<String, usize> = HashMap::new();

    for song in songs.clone() {
        for mood in song.mood {
            mood_hash.entry(mood).and_modify(|count| *count += 1).or_insert(0);
        }
    }

    let mut vec_moods = hashmap_to_vec_truple::<String, usize>(mood_hash);

    vec_moods.sort_by(|(a_key, a_val), (b_key, b_val)| {
        b_val.cmp(&a_val).then_with(|| a_key.cmp(&b_key))
    });

    array_truncate(&mut vec_moods, args.length);

    print_table::<String, usize>(headers, vec_moods);

    Ok(())
}

fn print_table<K: fmt::Display, T: fmt::Display>(headers: Row, data: Vec<(K, T)>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP);
    table.add_row(headers);

    for (key, value) in data {
        let row = row![key, value];
        table.add_row(row);
    }

    table.printstd();
}

fn get_songs_or_albums(album: bool, filters: SongDataFilter) -> Vec<SongData> {
    filters.filter(match album {
        true => get_albums().unwrap(),
        false => get_songs().unwrap(),
    })
}

fn get_count_headers(count_name: &str, album: bool) -> Row {
    match album {
        true => row![count_name, "# of Albums"],
        false => row![count_name, "# of Songs"],
    }
}
