use clap::Args;
use prettytable::{format, row, Table};
use std::{collections::HashMap, error::Error};

use crate::utils::songs::{get_songs, SongDataFilter};

#[derive(Args)]
pub struct TimeArgs {
    #[clap(short = 'd', long = "decade")]
    decade: Option<u16>,
    #[clap(short = 'y', long = "year")]
    year: Option<i32>,
    #[clap(short = 'm', long = "month")]
    month: Option<u32>,
    #[clap(short = 'g', long = "genre")]
    genre: Option<String>,
    #[clap(short = 'A', long = "artist")]
    artist: Option<String>,
    #[clap(short = 'a', long = "album")]
    album: Option<String>,
}

pub fn times_of_music(args: TimeArgs) -> Result<(), Box<dyn Error>> {
    let songs = get_songs()?;
    let total_song_length;
    let max_song;
    let min_song;
    let mut album_lengths: HashMap<String, u64> = HashMap::new();
    let max_album;
    let min_album;

    let filter: SongDataFilter = SongDataFilter {
        month: args.month,
        year: args.year,
        genre: args.genre,
        artist: args.artist,
        ..Default::default()
    };

    let filtered_songs: Vec<_> = filter.filter(songs);
    let track_times: Vec<u64> = filtered_songs
        .iter()
        .map(|s| {
            let length = album_lengths.entry(s.album_title.clone()).or_insert(0);
            *length += s.track_length;

            s.track_length
        })
        .collect();
    let hash_values = album_lengths.values().cloned();

    min_song = track_times.iter().min().unwrap();
    max_song = track_times.iter().max().unwrap();
    total_song_length = track_times.iter().sum();

    min_album = hash_values.clone().min().unwrap();
    max_album = hash_values.clone().max().unwrap();
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP);
    table.add_row(row!["Name", "Times"]);

    table.add_row(row!["Shortest album", convert_sec_to_fmt_time(min_album)]);
    table.add_row(row![
        "Avg album length",
        convert_sec_to_fmt_time(total_song_length / (hash_values.len() as u64))
    ]);
    table.add_row(row!["Longest album", convert_sec_to_fmt_time(max_album)]);
    table.add_row(row!["Shortest song", convert_sec_to_fmt_time(*min_song)]);
    table.add_row(row![
        "Avg song length",
        convert_sec_to_fmt_time(total_song_length / (filtered_songs.len() as u64))
    ]);
    table.add_row(row!["Longest song", convert_sec_to_fmt_time(*max_song)]);
    table.add_row(row!["Total song length", convert_sec_to_fmt_time(total_song_length)]);

    table.printstd();

    Ok(())
}

fn convert_sec_to_fmt_time(sec: u64) -> String {
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
