use std::error::Error;

use clap::Args;

use crate::utils::songs::load_song_tag;

#[derive(Args)]
pub struct LyricsArgs {
    /// Filename
    name: String,
}

pub fn get_track_lyrics(args: LyricsArgs) -> Result<(), Box<dyn Error>> {
    let song_tag = load_song_tag(&args.name);

    for section in song_tag.lyrics {
        let lines = section.split('\n');

        for line in lines {
            println!("{}", line);
        }

        println!("{}", "");
    }

    Ok(())
}
