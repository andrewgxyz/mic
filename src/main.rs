/*
*   Music Analytics
*
*   I want a piece of software that will let me do some data visualization on my music collection
*
*   I know some of this requires that the music data needs to be filled but this is my music data
*
*   Do I wanna create a graphical environment for this
*
*   What do I want to know?
*       - How many songs are in each decade
*       - How many songs are in each year
*       - Number of albums per genres
*
*/

use clap::{Parser, Subcommand};

mod utils;
mod commands;

use crate::commands::count::*;
use crate::commands::playlist::*;
use crate::commands::time::*;
use crate::commands::wtp::*;

#[derive(Parser)]
#[command(
    name = "mic", 
    author = "andrewgxyz", 
    about = "A general tool around manipulating local music collection."
)]
struct Cli {
    #[command(subcommand)]
    pub command: Command
}

#[derive(Subcommand)]
enum Command {
    /// Count number of songs by group
    #[command(arg_required_else_help = false)]
    Count(CountArgs),

    /// Generates a playlist based on certain filters
    Playlist(PlaylistArgs),

    /// Gives Insight runtimes of an Album or Collection
    Time(TimeArgs),

    /// What records to play based on release ranges
    Wtp(WtpArgs),
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Count(args) => count_music(args)?,
        Command::Playlist(args) => generate_playlist(args)?,
        Command::Time(args) => times_of_music(args)?,
        Command::Wtp(args) => wtpn(args)?,
    };

    Ok(())
}
