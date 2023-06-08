use glob::{glob_with, MatchOptions};
use lofty::{Probe, TaggedFileExt, ItemKey, Tag, TagExt, AudioFile, Accessor};
use std::{env, error::Error, cmp::{min, max}};
use clap::{Parser, Subcommand, Args};

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

#[derive(Debug)]
struct SongData {
    // Track Info
    _album_artist: String,
    _album_title: String,
    _catalog_number: String,
    _genre: Vec<String>,
    _mood: Vec<String>,
    _movement: String,
    _movement_number: String,
    _movement_total: String,
    _track_artist: String,
    _track_length: u64,
    _track_number: String,
    _track_title: String,
    _track_total: String,
    recording_date: String,
    // Details and Comments
    _comment: String,
    _copyright_message: String,
    _description: String,
    _label: String,
    _license: String,
    _parental_advisory: String,
    // Production Credits
    _arranger: Vec<String>,
    _composer: Vec<String>,
    _conductor: Vec<String>,
    _director: Vec<String>,
    _engineer: Vec<String>,
    _involved_people: Vec<String>,
    _language: String,
    _length: String,
    _lyricist: Vec<String>,
    _lyrics: String,
    _mix_dj: Vec<String>,
    _mix_engineer: Vec<String>,
    _musician_credits: String,
    _performer: Vec<String>,
    _producer: Vec<String>,
    _publisher: String,
    _remixer: String,
    _script: String,
    _work: String,
    _writer: Vec<String>,
}

#[derive(Debug)]
struct Count {
    index: String,
    count: i32
}

#[derive(Subcommand)]
enum Command {
    // Count number of songs by group
    #[command(arg_required_else_help = true)]
    Count(CountArgs),
    // Count number of songs by group
    Time(TimeArgs),
    Fix(TimeArgs)
}

#[derive(Args)]
struct CountArgs {
    #[clap(short = 'd', long = "decade")]
    decade: bool,
    #[clap(short = 'y', long = "year")]
    year: bool,
    #[clap(short = 'm', long = "month")]
    month: bool,
    #[clap(short = 'g', long = "genre")]
    genre: bool,
    #[clap(short = 'a', long = "album")]
    album: bool,
    #[clap(short = 'A', long = "artist")]
    artist: bool,
    #[clap(short = 'c', long = "composer")]
    composer: bool,
}

#[derive(Args)]
struct TimeArgs {
    #[clap(short = 'd', long = "decade")]
    decade: Option<u16>,
    #[clap(short = 'y', long = "year")]
    year: Option<u16>,
    #[clap(short = 'g', long = "genre")]
    genre: Option<String>,
    #[clap(short = 'A', long = "artist")]
    artist: Option<String>,
    #[clap(short = 'a', long = "album")]
    album: Option<String>,
}

#[derive(Parser)]
struct CliArgs {
    #[command(subcommand)]
    pub command: Command
}

fn get_tag (tag: &Tag, key: &ItemKey) -> String {
    tag.get_string(key).unwrap_or("None").to_string()
}

fn find_index_and_count_up (arr: &mut Vec<Count>, index: String) {
    if let Some(pos) = arr.iter().position(|e| e.index.eq(&index)) {
        arr[pos].count += 1;
    } else {
        arr.push(Count { index, count: 1 })
    }
}

fn get_song_list () -> Result<Vec<SongData>, Box<dyn Error>> {
    let home = env::var_os("HOME").unwrap().into_string().unwrap();
    let mut songs: Vec<SongData> = vec!(); 

    get_song_vec_data(&mut songs, format!("{}/music/*/*/*[.flac,.mp3,.wav]", &home))?;
    Ok(songs)
}

fn get_album_list () -> Result<Vec<SongData>, Box<dyn Error>> {
    let home = env::var_os("HOME").unwrap().into_string().unwrap();
    let mut albums: Vec<SongData> = vec!(); 

    get_song_vec_data(&mut albums, format!("{}/music/*/*/01-*[.flac,.mp3,.wav]", &home))?;
    Ok(albums)
}

fn times_of_music (args: TimeArgs) -> Result<(), Box<dyn Error>> {
    let mut songs = get_song_list()?;
    let mut total_song_length = 0;
    let mut max_song = 0;
    let mut min_song = 99999999;
    let mut albums: Vec<String> = vec![];
    let mut album_lengths: Vec<u64> = vec![];
    let mut max_album = 0;
    let mut min_album = 99999999;
    let mut artists: Vec<String> = vec![];
    let mut genres: Vec<String> = vec![];

    if args.artist.is_some() {
        let artist = args.artist.clone().unwrap();
        songs.retain(|e| e._track_artist == artist);
    } else if args.year.is_some() {
        let artist = args.artist.clone().unwrap();
        songs.retain(|e| e._track_artist == artist);
    }

    for song in &songs {
        total_song_length += song._track_length;
        min_song = min(min_song, song._track_length);
        max_song = max(max_song, song._track_length);

        song._genre.iter().for_each(|a| {
            if !genres.contains(a) {
                genres.push(a.to_string());
            }
        });

        if albums.contains(&song._album_title) {
            let index = albums.iter().position(|e| e.eq(&song._album_title)).unwrap();

            album_lengths[index] += song._track_length;
        } else {
            albums.push(song._album_title.to_string());
            album_lengths.push(song._track_length);
        }

        if !artists.contains(&song._track_artist) {
            artists.push(song._track_artist.to_string());
        }
    }

    for album in album_lengths.clone() {
        max_album = max(album, max_album);
        min_album = min(album, min_album);
    }

    println!("{0: <20} | {1: <10}", "Name", "Totals");
    println!("{0: <20} | {1: <10}", "----------", "----------");
    println!("{0: <20} | {1: <10}", "Num of Songs", &songs.len().to_string());
    println!("{0: <20} | {1: <10}", "Num of Albums", &albums.len().to_string());
    println!("{0: <20} | {1: <10}", "Avergae Album Length", convert_sec_to_fmt_time(total_song_length / (albums.len() as u64)));
    println!("{0: <20} | {1: <10}", "Longest album", convert_sec_to_fmt_time(max_album));
    println!("{0: <20} | {1: <10}", "Shortest album", convert_sec_to_fmt_time(min_album));
    if args.artist.is_none() {
        println!("{0: <20} | {1: <10}", "Num of Artists", &artists.len().to_string());
    }
    println!("{0: <20} | {1: <10}", "Num of Genres", &genres.len().to_string());
    println!("{0: <20} | {1: <10}", "Avg song length", convert_sec_to_fmt_time(total_song_length / (songs.len() as u64)));
    println!("{0: <20} | {1: <10}", "Longest song", convert_sec_to_fmt_time(max_song));
    println!("{0: <20} | {1: <10}", "Shortest song", convert_sec_to_fmt_time(min_song));
    println!("{0: <20} | {1: <10}", "Total song length", convert_sec_to_fmt_time(total_song_length));

    Ok(())
}

fn count_music (args: CountArgs) -> Result<(), Box<dyn Error>> {
    let mut count_title = "# of Songs";
    let mut num_songs_by_decade: Vec<Count> = vec!();
    let mut subject_title = "Decade";
    let songs: Vec<SongData>;

    if args.album {
        count_title = "# of Albums";
        songs = get_album_list()?;
    } else {
        songs = get_song_list()?;
    }

    if args.genre {
        subject_title = "Genre";
    }

    if args.year {
        subject_title = "Year";
    }

    if args.month {
        subject_title = "Month";
    }

    if args.artist {
        subject_title = "Artist";
    }

    if args.composer {
        subject_title = "Composer";
    }

    for song in songs {
        if args.genre {
            for genre in song._genre {
                find_index_and_count_up(&mut num_songs_by_decade, genre);
            }

            continue;
        } else if args.composer {
            for composer in song._composer {
                find_index_and_count_up(&mut num_songs_by_decade, composer);
            }
        } else {
            let date_vec: Vec<String> = song.recording_date.split('-').map(|e| e.to_string()).collect();
            let mut index = String::from(&date_vec[0]);

            if args.month {
                index = String::from(&date_vec[1])
            } else if args.artist {
                index = String::from(&song._track_artist);
            } else if !args.year {
                index.replace_range(3..4, "0");
            }

            find_index_and_count_up(&mut num_songs_by_decade, index);
        }
    }

    num_songs_by_decade.sort_by(|a,b| a.index.cmp(&b.index));

    println!("{0: <30} | {1: <10}", subject_title, count_title);
    println!("{0: <30} | {1: <10}", "----------", "----------");
    for num_songs in num_songs_by_decade {
        println!("{0: <30} | {1: <10}", num_songs.index, num_songs.count);
    }

    Ok(())
}

fn convert_sec_to_fmt_time (sec: u64) -> String {
    let hrs: i32 = (sec / 3600) as i32; 
    let mut min: i32 = (sec / 60) as i32;

    if hrs > 0 {
        min = (sec as i32 - (hrs * 3600)) / 60;
        if hrs > 9 {
            return format!("{:03.0}:{:02.0}:{:02.0}", hrs, min, sec % 60 )
        }

        return format!("{:02.0}:{:02.0}:{:02.0}", hrs, min, sec % 60 )
    }

    format!("{:02.0}:{:02.0}", min, sec % 60 )
}

fn get_song_vec_data (arr: &mut Vec<SongData>, glob_pattern: String) -> Result<(), Box<dyn Error>> {
    let globs = glob_with(&glob_pattern, MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    })?;

    for entry in globs.flatten() {
        let filename = entry.display().to_string();

        let tagged_file = Probe::open(filename)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag() {
            Some(primary) => primary,
            None => tagged_file.first_tag().expect("ERROR: No tags found!")
        };

        let album_artist = get_tag(tag, &ItemKey::AlbumArtist);
        let album_title = get_tag(tag, &ItemKey::AlbumTitle);
        let arranger = get_tag(tag, &ItemKey::Arranger).split('/').map(|s| s.to_string()).collect();
        let catalog_number = get_tag(tag, &ItemKey::CatalogNumber);
        let comment = get_tag(tag, &ItemKey::Comment);
        let composer = get_tag(tag, &ItemKey::Composer).split('/').map(|s| s.to_string()).collect();
        let conductor = get_tag(tag, &ItemKey::Conductor).split('/').map(|s| s.to_string()).collect();
        let copyright_message = get_tag(tag, &ItemKey::CopyrightMessage);
        let description = get_tag(tag, &ItemKey::Description);
        let director = get_tag(tag, &ItemKey::Director).split('/').map(|s| s.to_string()).collect();
        let engineer = get_tag(tag, &ItemKey::Engineer).split('/').map(|s| s.to_string()).collect();
        let genre: Vec<String> = get_tag(tag, &ItemKey::Genre).split('/').map(|s| s.to_string()).collect();
        let involved_people = get_tag(tag, &ItemKey::InvolvedPeople).split('/').map(|s| s.to_string()).collect();
        let label = get_tag(tag, &ItemKey::Label);
        let language = get_tag(tag, &ItemKey::Language);
        let length = get_tag(tag, &ItemKey::Length);
        let license = get_tag(tag, &ItemKey::License);
        let lyricist = get_tag(tag, &ItemKey::Lyricist).split('/').map(|s| s.to_string()).collect();
        let lyrics = get_tag(tag, &ItemKey::Lyrics);
        let mix_dj = get_tag(tag, &ItemKey::MixDj).split('/').map(|s| s.to_string()).collect();
        let mix_engineer = get_tag(tag, &ItemKey::MixEngineer).split('/').map(|s| s.to_string()).collect();
        let mood: Vec<String>= get_tag(tag, &ItemKey::Mood).split('/').map(|s| s.to_string()).collect();
        let movement = get_tag(tag, &ItemKey::Movement);
        let movement_number = get_tag(tag, &ItemKey::MovementNumber);
        let movement_total = get_tag(tag, &ItemKey::MovementTotal);
        let musician_credits = get_tag(tag, &ItemKey::MusicianCredits).split('/').map(|s| s.to_string()).collect();
        let parental_advisory = get_tag(tag, &ItemKey::ParentalAdvisory);
        let performer = get_tag(tag, &ItemKey::Performer).split('/').map(|s| s.to_string()).collect();
        let producer = get_tag(tag, &ItemKey::Producer).split('/').map(|s| s.to_string()).collect();
        let publisher = get_tag(tag, &ItemKey::Publisher);
        let recording_date = get_tag(tag, &ItemKey::RecordingDate);
        let remixer = get_tag(tag, &ItemKey::Remixer);
        let script = get_tag(tag, &ItemKey::Script);
        let track_length = tagged_file.properties().duration().as_secs();
        let track_artist = get_tag(tag, &ItemKey::TrackArtist);
        let track_number = get_tag(tag, &ItemKey::TrackNumber);
        let track_title = get_tag(tag, &ItemKey::TrackTitle);
        let track_total = get_tag(tag, &ItemKey::TrackTotal);
        let work = get_tag(tag, &ItemKey::Work);
        let writer = get_tag(tag, &ItemKey::Writer).split('/').map(|s| s.to_string()).collect();

        arr.push(SongData {
            _album_artist: album_artist,
            _album_title: album_title,
            _arranger: arranger,
            _catalog_number: catalog_number,
            _comment: comment,
            _composer: composer,
            _conductor: conductor,
            _copyright_message: copyright_message,
            _description: description,
            _director: director,
            _engineer: engineer,
            _genre: genre,
            _involved_people: involved_people,
            _label: label,
            _language: language,
            _length: length,
            _license: license,
            _lyricist: lyricist,
            _lyrics: lyrics,
            _mix_dj: mix_dj,
            _mix_engineer: mix_engineer,
            _mood: mood,
            _movement: movement,
            _movement_number: movement_number,
            _movement_total: movement_total,
            _musician_credits: musician_credits,
            _parental_advisory: parental_advisory,
            _performer: performer,
            _producer: producer,
            _publisher: publisher,
            recording_date,
            _remixer: remixer,
            _script: script,
            _track_length: track_length,
            _track_artist: track_artist,
            _track_number: track_number,
            _track_title: track_title,
            _track_total: track_total,
            _work: work,
            _writer: writer,
        });
    }

    Ok(())
}

fn fix_tag () -> Result<(), Box<dyn Error>>  {
    let home = env::var_os("HOME").unwrap().into_string().unwrap();

    let globs = glob_with(format!("{}/music/*/*/*[.flac,.mp3,.wav]", &home).as_str(), MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    })?;

    for entry in globs.flatten() {
        let filename = entry.display().to_string();

        let mut tagged_file = Probe::open(&filename)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag_mut() {
            Some(primary) => primary,
            None => {
                if let Some(first_tag) = tagged_file.first_tag_mut() {
                    first_tag
                } else {
                    let tag_type = tagged_file.primary_tag_type();

                    eprintln!("WARN: No tags found, creating a new tag of type `{tag_type:?}`");
                    tagged_file.insert_tag(Tag::new(tag_type));

                    tagged_file.primary_tag_mut().unwrap()
                }
            }
        };

        // let mf_tag = metaflac::Tag::read_from_path(&filename)?;

        // let arranger = get_tag(tag, &ItemKey::Arranger).replace(", ", "/");
        // let composer = get_tag(tag, &ItemKey::Composer).replace(", ", "/");
        // let conductor = get_tag(tag, &ItemKey::Conductor).replace(", ", "/");
        // let director = get_tag(tag, &ItemKey::Director).replace(", ", "/");
        // let engineer = get_tag(tag, &ItemKey::Engineer).replace(", ", "/");
        let genre = get_tag(tag, &ItemKey::Genre)
            .replace(" Concious Hip Hop", "Conscious Hip Hop")
            .replace(" Experimental Hip Hop", "Experimental Hip Hop")
            .replace(" Space Age Pop", "Space Age Pop")
            .replace("Abmient", "Ambient")
            .replace("Arabic Folk Music", "Arabic Folk")
            .replace("Art Punk Art Rock", "Art Punk/Art Rock")
            .replace("Dowmtempo", "Downtempo")
            .replace("Dance-Pop Blue-Eyed Soul", "Dance-Pop/Blue-Eyed Soul")
            .replace("Eletro House", "Electro-House")
            .replace("Eletro-Disco", "Electro-Disco")
            .replace("Electro House", "Electro-House")
            .replace("Electro Swing", "Electro-Swing")
            .replace("Glictch", "Glitch")
            .replace("Intrumental Hip Hop", "Instrumental Hip Hop")
            .replace("Jazz Fushion", "Jazz Fusion")
            .replace("Lo-Fi", "Lofi")
            .replace("Neo Psychedelia", "Neo-Psychedelia")
            .replace("Neo-Psychodelia", "Neo-Psychedelia")
            .replace("Nu Metal", "Nu-Metal")
            .replace("Nu Jazz", "Nu-Jazz")
            .replace("Post Rock", "Post-Rock")
            .replace("Post Punk", "Post-Punk")
            .replace("Post Industrial", "Post-Industrial")
            .replace("Progressive PoplArt Pop", "Progressive Pop/Art Pop")
            .replace("Psychodelic Pop", "Psychedelic Pop")
            .replace("Sunshice Pop", "Sunshine Pop")
            .replace("Synphonic Metal", "Symphonic Metal")
            .replace("Synth Pop", "Synthpop")
            .replace("Synth-Pop", "Synthpop")
            .replace("Synth Fuck", "Synth Punk")
            .replace("Video Game Music", "Video Game");
        // let involved_people = get_tag(tag, &ItemKey::InvolvedPeople).replace(", ", "/");
        // let lyricist = get_tag(tag, &ItemKey::Lyricist).replace(", ", "/");
        // let mix_dj = get_tag(tag, &ItemKey::MixDj).replace(", ", "/");
        // let mix_engineer = get_tag(tag, &ItemKey::MixEngineer).replace(", ", "/");
        // let mood = get_tag(tag, &ItemKey::Mood).replace(", ", "/");
        // let musician_credits = get_tag(tag, &ItemKey::MusicianCredits).replace(", ", "/");
        // let performer = get_tag(tag, &ItemKey::Performer).replace(", ", "/");
        // let producer = get_tag(tag, &ItemKey::Producer).replace(", ", "/");
        // let writer = get_tag(tag, &ItemKey::Writer).replace(", ", "/");

        // mf_tag.set_vorbis("")

        tag.set_genre(genre);
        tag.save_to_path(&filename).unwrap();
        println!("Genres have been updated for: {}", filename);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    match args.command {
        Command::Count(args) => count_music(args)?,
        Command::Time(args) => times_of_music(args)?,
        Command::Fix(args) => fix_tag()?,
    };

    Ok(())
}
