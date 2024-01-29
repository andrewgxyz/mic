use std::error::Error;

use clap::Args;
use image::ImageBuffer;

use crate::utils::covers::{get_album_covers, AlbumCoverData, AlbumCoverDataFilter};

#[derive(Args)]
pub struct AccgArgs {
    /// Name of the Artist ex. "Green Day"
    #[clap(short = 'a', long = "artist")]
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

    /// Get list by current week
    #[clap(short = 'w', long = "week")]
    week: bool,

    /// Filename for the playlist output without extension
    name: Option<String>,
}

const MAX_WIDTH: u32 = 3840;
const MAX_HEIGHT: u32 = 2160;

fn create_collage(images: Vec<AlbumCoverData>) -> Result<image::DynamicImage, Box<dyn Error>> {
    // Get the dimensions of the first image to determine the size of the collage
    let grid_size = (images.len() as f64).sqrt().ceil() as usize;
    let rows = (images.len() as f64 / grid_size as f64).ceil() as usize;
    let cols = grid_size;

    let col_size = MAX_WIDTH / cols as u32;
    let row_size = MAX_HEIGHT / rows as u32;

    let size = col_size.min(row_size);

    // Create a new image for the collage
    let center_origin = cols as u32 * size as u32;
    let mut collage = image::DynamicImage::new_rgb8(center_origin, MAX_HEIGHT);

    println!("{}x{}", cols, rows);
    println!("{}x{}", size, size);

    for y in 0..rows {
        for x in 0..cols {
            let index = x + (cols * y);
            if index >= images.len() {
                break;
            }

            let img = images[index].clone();
            let pos_x: i64 = size as i64 * x as i64;
            let pos_y: i64 = size as i64 * y as i64;
            let gen_img = image::DynamicImage::ImageRgb8(
                ImageBuffer::from_vec(img.image.width, img.image.width, img.image.pixels).unwrap(),
            )
            .resize(size, size, image::imageops::FilterType::Triangle);
            image::imageops::overlay(&mut collage, &gen_img, pos_x, pos_y);
        }
    }

    Ok(collage)
}

pub fn accg(args: AccgArgs) -> Result<(), Box<dyn Error>> {
    let covers = get_album_covers()?;
    let filter = AlbumCoverDataFilter {
        month: args.month,
        year: args.year,
        genre: args.genre,
        moods: args.moods,
        artist: args.artist,
        decade: args.decade,
        week: args.week,
        ..Default::default()
    };

    let mut filtered = filter.filter(covers);
    filtered.sort_by(|a, b| a.album_data._album_artist.cmp(&b.album_data._album_artist));

    let collage = create_collage(filtered)?;

    collage.save("merged_img.png").expect("Failed to save new image");

    Ok(())
}
