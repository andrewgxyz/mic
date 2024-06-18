use std::{error::Error, collections::HashSet};

use clap::Args;

use crate::utils::songs::get_songs;

#[derive(Args)]
pub struct MissingArgs {
    /// Exclude list of tags separated by commas
    #[clap(short = 't', long = "tag")]
    tag: Option<String>,
}

pub fn get_missing_tag(args: MissingArgs) -> Result<(), Box<dyn Error>> {
    let songs = get_songs()?;

    let list_missing_tags = songs.iter().filter(|s| s.lyrics.len() < 1);

    let vec_dirs: HashSet<_> = list_missing_tags.map(|e| &e.dir).collect();

    for dir in vec_dirs {
        println!("{}", dir);
    }

    Ok(())
}
