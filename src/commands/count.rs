use std::error::Error;

use clap::Args;

use crate::utils::songs::{get_songs, get_albums, SongData, SongDataFilter};

#[derive(Debug)]
pub struct Count {
    index: String,
    count: i32
}

#[derive(Args)]
pub struct CountArgs {
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
    album: bool,
}

pub fn count_music (args: CountArgs) -> Result<(), Box<dyn Error>> {
    let mut count_title = "# of Songs";
    let mut num_songs_by_decade: Vec<Count> = vec!();
    let mut subject_title = "Decade";
    let mut songs: Vec<SongData>;

    if args.album {
        count_title = "# of Albums";
        songs = get_albums()?;
    } else {
        songs = get_songs()?;
    }

    let filter: SongDataFilter = SongDataFilter {
        month: args.month,
        year: args.year,
        genre: args.genre,
        artist: args.artist,
        ..Default::default()
    };

    let mut filtered_songs: Vec<_> = filter.filter(songs);

    // if args.genre.as_deref() {
    //     subject_title = "Genre";
    // }

    if args.year.is_some() {
        subject_title = "Year";
    }

    if args.month.is_some() {
        subject_title = "Month";
    }

    // if args.artist.is_some() {
    //     subject_title = "Artist";
    // }

    num_songs_by_decade.sort_by(|a,b| a.index.cmp(&b.index));

    println!("{0: <5} | {1: <40}", count_title, subject_title);
    println!("{0: <5} | {1: <40}", "-----", "----------");
    for num_songs in num_songs_by_decade {
        println!("{0: <5} | {1: <40}", num_songs.count, num_songs.index);
    }

    Ok(())
}
