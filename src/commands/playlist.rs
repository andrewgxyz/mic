use std::error::Error;

use clap::Args;
use rand::seq::SliceRandom;

use crate::utils::{
    songs::{SongData, get_songs, SongDataFilter},
    date::parse_string_to_datetime, data::array_truncate
};

#[derive(Args)]
pub struct PlaylistArgs {
    /// Album Title ex. "Dookie"
    #[clap(short = 'a', long = "album")]
    album: Option<String>,

    /// Name of the Artist ex. "Green Day"
    #[clap(short = 'A', long = "artist")]
    artist: Option<String>,

    /// Genre name ex. "Synth,Metal,Punk"
    #[clap(short = 'g', long = "genre")]
    genre: Option<String>,

    /// Mood names ex. "eclectic,warm,dark"
    #[clap(short = 'M', long = "moods")]
    moods: Option<String>,

    /// Day of release ex. 2
    #[clap(short = 'd', long = "day")]
    day: Option<u16>,

    /// Decade of release ex. 1980
    #[clap(short = 'D', long = "decade")]
    decade: Option<u16>,

    /// Month of release ex. 5
    #[clap(short = 'm', long = "month")]
    month: Option<u32>,

    /// Year of release ex. 2020
    #[clap(short = 'y', long = "year")]
    year: Option<i32>,

    /// Set max length for playlist
    #[clap(short = 'l', long = "length")]
    length: Option<usize>,

    /// Filter by track number
    #[clap(short = 't', long = "track")]
    track: Option<String>,

    /// Randomize order of list
    #[clap(short = 'r', long = "random")]
    random: bool,

    /// Get list by current week
    #[clap(short = 'w', long = "week")]
    week: bool,

    /// Get list by current week
    #[clap(short = 'W', long = "words")]
    words: Option<String>,

    /// Filename for the playlist output without extension
    name: Option<String>,
}

pub fn generate_playlist (args: PlaylistArgs) -> Result<(), Box<dyn Error>> {
    let songs: Vec<SongData> = get_songs()?;
    let filter: SongDataFilter = SongDataFilter {
        month: args.month,
        year: args.year,
        genre: args.genre,
        moods: args.moods,
        artist: args.artist,
        decade: args.decade,
        week: args.week,
        words: args.words,
        track: args.track,
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

    let mut song_files: Vec<String> = filtered_songs.iter().map(|s| s.filename.replace("/home/andrew/music/", "").to_string()).collect();

    array_truncate::<String>(&mut song_files, args.length);

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
