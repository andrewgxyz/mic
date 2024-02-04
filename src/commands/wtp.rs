use std::error::Error;

use clap::Args;

use crate::utils::{
    date::parse_string_to_yearless_date,
    songs::{get_albums, SongData, SongDataFilter},
};

#[derive(Args)]
pub struct WtpArgs {
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

    /// Get list by current week
    #[clap(short = 'w', long = "week")]
    week: bool,
}

pub fn wtpn(args: WtpArgs) -> Result<(), Box<dyn Error>> {
    let songs: Vec<SongData> = get_albums()?;
    let filter: SongDataFilter = SongDataFilter {
        month: args.month,
        year: args.year,
        week: args.week,
        ..Default::default()
    };

    let mut filtered_songs: Vec<_> = filter.filter(songs);

    filtered_songs.sort_by(|a, b| {
        let a_yearless = parse_string_to_yearless_date(&a.recording_date);
        let b_yearless = parse_string_to_yearless_date(&b.recording_date);
        a_yearless.cmp(&b_yearless)
    });

    for song in filtered_songs {
        println!("{} {} - {}", song.recording_date, song._track_artist, song._album_title);
    }

    Ok(())
}
