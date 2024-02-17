use std::error::Error;

use clap::Args;
use prettytable::{format, row, Table};
use serde_json::Value;

use crate::utils::{songs::{load_song_tag, get_songs, SongData}, data::string_to_vec};

#[derive(Args)]
pub struct InfoArgs {
    /// Exclude list of tags separated by commas
    #[clap(short = 'x', long = "exclude")]
    exclude: Option<String>,

    /// Only show missing tags
    #[clap(short = 'm', long = "missing")]
    missing: bool,

    /// Filename
    name: String,
}

fn is_tag_excluded(tagname: String, excludes: &Vec<String>) -> bool {
    excludes.contains(&tagname)
}

pub fn get_track_info(args: InfoArgs) -> Result<(), Box<dyn Error>> {
    // let songs = get_songs()?;

    // let check: Vec<_> = songs.iter().filter(|s| {
    //     let key = args.name.as_str();
    //     match key {
    //         "album_artist" => s.album_artist.is_empty(),
    //         "album_title" => s.album_title.is_empty(),
    //         "arranger" => s.arranger.is_empty(),
    //         "catalog_number" => s.catalog_number.is_empty(),
    //         "comment" => s.comment.is_empty(),
    //         "composer" => s.composer.is_empty(),
    //         "conductor" => s.conductor.is_empty(),
    //         "copyright_message" => s.copyright_message.is_empty(),
    //         "description" => s.description.is_empty(),
    //         "director" => s.director.is_empty(),
    //         "engineer" => s.engineer.is_empty(),
    //         "genre" => {
    //             println!("{}", s.genre.is_empty());
    //             s.genre.is_empty()
    //         },
    //         "involved_people" => s.involved_people.is_empty(),
    //         "label" => s.label.is_empty(),
    //         "language" => s.language.is_empty(),
    //         "length" => s.length.is_empty(),
    //         "license" => s.license.is_empty(),
    //         "lyricist" => s.lyricist.is_empty(),
    //         "lyrics" => s.lyrics.is_empty(),
    //         "mix_dj" => s.mix_dj.is_empty(),
    //         "mix_engineer" => s.mix_engineer.is_empty(),
    //         "mood" => s.mood.is_empty(),
    //         "movement" => s.movement.is_empty(),
    //         "movement_number" => s.movement_number.is_empty(),
    //         "movement_total" => s.movement_total.is_empty(),
    //         "musician_credits" => s.musician_credits.is_empty(),
    //         "parental_advisory" => s.parental_advisory.is_empty(),
    //         "performer" => s.performer.is_empty(),
    //         "producer" => s.producer.is_empty(),
    //         "publisher" => s.publisher.is_empty(),
    //         "recording_date" => s.recording_date.is_empty(),
    //         "remixer" => s.remixer.is_empty(),
    //         "script" => s.script.is_empty(),
    //         "track_length" => s.track_length < 1,
    //         "track_artist" => s.track_artist.is_empty(),
    //         "track_number" => s.track_number.is_empty(),
    //         "track_title" => s.track_title.is_empty(),
    //         "track_total" => s.track_total.is_empty(),
    //         "work" => s.work.is_empty(),
    //         "writer" => s.writer.is_empty(),
    //         _ => true
    //     }
    // }).collect();

    // println!("{:?}", check);

    let song_tag = load_song_tag(&args.name);

    let headers = row!["Tag", "Value"];

    let song: Value = serde_json::to_value(&song_tag).unwrap();
    let excludes: Vec<String> = match args.exclude {
        Some(exclude) => string_to_vec(exclude, ","),
        None => vec![]
    };

    if let Some(map) = song.as_object() {
        let mut table_rows: Vec<(&str, &Value)> = vec![];

        for (key, value) in map {
            table_rows.push((key, value));
        }

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP);
        table.add_row(headers);

        for (key, value) in table_rows {
            if is_tag_excluded(key.to_string(), &excludes) {
                continue;
            }

            match value {
                Value::Array(arr) => {
                    if arr.len() > 0 && args.missing {
                        continue;
                    }
                },
                Value::String(s) => {
                    if !s.is_empty() && args.missing {
                        continue;
                    }
                },
                Value::Number(n) => {
                    if n.as_u64().unwrap() != 0 && args.missing {
                        continue;
                    }
                },
                _ => {}
            }

            table.add_row(row![key, value]);
        }

        table.printstd();
    }

    Ok(())
}
