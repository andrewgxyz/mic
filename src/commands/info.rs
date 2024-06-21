use std::error::Error;

use clap::Args;
use prettytable::{format, row, Table};
use serde_json::Value;

use crate::utils::{data::string_to_vec, songs::load_song_tag};

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
