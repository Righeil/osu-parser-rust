use std::collections::HashMap;
use crate::magic;
use crate::osu::Mode;
use crate::osu::settings::{
    General, Editor, Metadata, Difficulty, Events
};

pub fn get_general(section: &Vec<String>) -> General {
    let mut data: HashMap<String, String> = HashMap::new();

    for line in section {
        get_key_value(line, &mut data)
    }

    let audio_filename = magic::get_value(&data, "AudioFilename", String::new());
    let preview_time = magic::get_value::<f64>(&data, "PreviewTime", 0.0);
    let mode = magic::get_value::<i8>(&data, "Mode", 4);

    let mode: Mode = match mode {
        0 => Mode::Osu,
        1 => Mode::Taiko,
        2 => Mode::Fruits,
        3 => Mode::Mania,
        _ => Mode::Unknown
    };

    General{
        audio_filename,
        preview_time,
        mode
    }
}

pub fn get_editor(section: &Vec<String>) -> Editor {
    Editor {}
}

pub fn get_difficulty(section: &Vec<String>) -> Difficulty {
    let mut data: HashMap<String, String> = HashMap::new();
    for line in section {
        get_key_value(line, &mut data)
    }

    let circle_size = magic::get_value::<f32>(&data, "CircleSize", 5.0);
    
    Difficulty {
        circle_size
    }
}

pub fn get_metadata(section: &Vec<String>) -> Metadata {
    let mut data: HashMap<String, String> = HashMap::new();

    for line in section {
        get_key_value(line, &mut data)
    }

    let title =          magic::get_value(&data, "Title", String::new());
    let title_unicode =  magic::get_value(&data, "TitleUnicode", String::new());
    let artist =         magic::get_value(&data, "Artist", String::new());
    let artist_unicode = magic::get_value(&data, "ArtistUnicode", String::new());
    let creator =        magic::get_value(&data, "Creator", String::new());
    let version =        magic::get_value(&data, "Version", String::new());

    Metadata{
        title,
        title_unicode,
        artist,
        artist_unicode,
        creator,
        version
    }
}

pub fn get_events(section: &Vec<String>) -> Events
{
    Events {}
}



fn get_key_value(line: &String, data: &mut HashMap<String, String>) {
    let key_value = line.split(":");
    let key_value = key_value.collect::<Vec<&str>>();

    if key_value.len() == 2 {
        data.insert(
            key_value[0].trim().to_string(), 
            key_value[1].trim().to_string()
        );
    }
}