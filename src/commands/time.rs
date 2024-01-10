use std::{env, error::Error, cmp::{min, max}};
use clap::Args;

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

pub fn times_of_music (args: TimeArgs) -> Result<(), Box<dyn Error>> {
    let mut songs = get_songs()?;
    let mut total_song_length = 0;
    let mut max_song = 0;
    let mut min_song = 99999999;
    let mut albums: Vec<String> = vec![];
    let mut album_lengths: Vec<u64> = vec![];
    let mut max_album = 0;
    let mut min_album = 99999999;
    let mut artists: Vec<String> = vec![];
    let mut genres: Vec<String> = vec![];

    let filter: SongDataFilter = SongDataFilter {
        month: args.month,
        year: args.year,
        genre: args.genre,
        artist: args.artist,
        ..Default::default()
    };

    let filtered_songs: Vec<_> = filter.filter(songs);

    for song in filtered_songs.clone() {
        total_song_length += song._track_length;
        min_song = min(min_song, song._track_length);
        max_song = max(max_song, song._track_length);

        song._genre.iter().for_each(|a| {
            if !genres.contains(a) {
                genres.push(a.to_string());
            }
        });

        if albums.contains(&song._album_title) {
            let index = albums.iter().position(|e| e.eq(&song._album_title)).unwrap();

            album_lengths[index] += song._track_length;
        } else {
            albums.push(song._album_title.to_string());
            album_lengths.push(song._track_length);
        }

        if !artists.contains(&song._track_artist) {
            artists.push(song._track_artist.to_string());
        }
    }

    for album in album_lengths.clone() {
        max_album = max(album, max_album);
        min_album = min(album, min_album);
    }

    println!("{0: <20} | {1: <10}", "Name", "Totals");
    println!("{0: <20} | {1: <10}", "----------", "----------");
    println!("{0: <20} | {1: <10}", "Num of Songs", filtered_songs.len().to_string());
    println!("{0: <20} | {1: <10}", "Num of Albums", &albums.len().to_string());
    println!("{0: <20} | {1: <10}", "Avergae Album Length", convert_sec_to_fmt_time(total_song_length / (albums.len() as u64)));
    println!("{0: <20} | {1: <10}", "Longest album", convert_sec_to_fmt_time(max_album));
    println!("{0: <20} | {1: <10}", "Shortest album", convert_sec_to_fmt_time(min_album));
    println!("{0: <20} | {1: <10}", "Num of Genres", &genres.len().to_string());
    println!("{0: <20} | {1: <10}", "Avg song length", convert_sec_to_fmt_time(total_song_length / (filtered_songs.len() as u64)));
    println!("{0: <20} | {1: <10}", "Longest song", convert_sec_to_fmt_time(max_song));
    println!("{0: <20} | {1: <10}", "Shortest song", convert_sec_to_fmt_time(min_song));
    println!("{0: <20} | {1: <10}", "Total song length", convert_sec_to_fmt_time(total_song_length));

    Ok(())
}

fn convert_sec_to_fmt_time (sec: u64) -> String {
    let hrs: i32 = (sec / 3600) as i32; 
    let mut min: i32 = (sec / 60) as i32;

    if hrs > 0 {
        min = (sec as i32 - (hrs * 3600)) / 60;
        if hrs > 9 {
            return format!("{:03.0}:{:02.0}:{:02.0}", hrs, min, sec % 60 )
        }

        return format!("{:02.0}:{:02.0}:{:02.0}", hrs, min, sec % 60 )
    }

    format!("{:02.0}:{:02.0}", min, sec % 60 )
}
