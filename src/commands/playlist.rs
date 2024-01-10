use std::error::Error;

use clap::Args;
use rand::seq::SliceRandom;

use crate::utils::{
    songs::{SongData, get_songs, SongDataFilter},
    date::parse_string_to_datetime
};

#[derive(Debug)]
pub struct Playlist {
}

#[derive(Args)]
pub struct PlaylistArgs {
    /// Album Title
    #[clap(short = 'a', long = "album")]
    album: Option<String>,

    /// Name of the Artist
    #[clap(short = 'A', long = "artist")]
    artist: Option<String>,

    /// Genre name
    #[clap(short = 'g', long = "genre")]
    genre: Option<String>,

    /// Day of release
    #[clap(short = 'd', long = "day")]
    day: Option<u16>,

    /// Decade of release
    #[clap(short = 'D', long = "decade")]
    decade: Option<u16>,

    /// Month of release
    #[clap(short = 'm', long = "month")]
    month: Option<u32>,

    /// Year of release
    #[clap(short = 'y', long = "year")]
    year: Option<i32>,

    /// Randomize order of list
    #[clap(short = 'r', long = "random")]
    random: bool,

    /// Filename for the playlist output
    name: Option<String>,
}

pub fn generate_playlist (args: PlaylistArgs) -> Result<(), Box<dyn Error>> {
    let songs: Vec<SongData> = get_songs()?;
    let filter: SongDataFilter = SongDataFilter {
        month: args.month,
        year: args.year,
        genre: args.genre,
        artist: args.artist,
        ..Default::default()
    };

    let mut filtered_songs: Vec<_> = filter.filter(songs);

    if args.random {
        let mut rnd = rand::thread_rng();
        filtered_songs.shuffle(&mut rnd);
    } else {
        filtered_songs.sort_by(|a,b| {
            let dt_a = parse_string_to_datetime(&a.recording_date).unwrap();
            let dt_b = parse_string_to_datetime(&b.recording_date).unwrap();
            dt_a.cmp(&dt_b) 
        });
    }

    let song_files: Vec<String> = filtered_songs.iter().map(|s| s.filename.replace("/home/andrew/music/", "").to_string()).collect();

    match args.name {
        Some(filename) => {
            let string_data = song_files.join("\n");

            match std::fs::write(format!("./{}.m3u", filename), &string_data) {
                Ok(_) => println!("Playlist has been created"),
                Err(e) => eprintln!("Something went wrong: {}", e)
            };
        },
        None => {
            for song in song_files {
                println!("{}", song);
            }
        }
    }

    Ok(())
}
