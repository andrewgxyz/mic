use std::error::Error;

use clap::Args;

use crate::utils::songs::load_song_tag;

#[derive(Args)]
pub struct InfoArgs {
    /// Filename
    name: String,
}

pub fn get_track_info(args: InfoArgs) -> Result<(), Box<dyn Error>> {
    let song_tag = load_song_tag(&args.name);

    println!("{:#?}", song_tag);

    Ok(())
}
