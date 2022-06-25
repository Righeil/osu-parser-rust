use crate::osu::{Mode, OverlayPosition, SampleSet};

pub struct Settings {
    pub general: General,
    pub editor: Editor,
    pub metadata: Metadata,
    pub difficulty: Difficulty,
    pub events: Events,
    pub colors: Vec<Color>,
}

pub struct General {
    pub audio_filename: String,
    pub audio_lead_in: f64,
    pub preview_time: f64,
    pub countdown: u32,
    pub sample_set: SampleSet,
    pub stack_leniency: f64,
    pub mode: Mode,
    pub letter_box_in_breaks: bool,
    pub use_skin_sprites: bool,
    pub overlay_position: OverlayPosition,
    pub skin_preference: String,
    pub epilepsy_warning: bool,
    pub countdown_offset: u32,
    pub special_style: bool,
    pub widescreen_storyboard: bool,
    pub samples_match_playback_rate: bool,
}

impl General {
    pub fn default() -> Self{
        General {
            audio_filename: String::new(),
            audio_lead_in: 0.0,
            preview_time: -1.0,
            countdown: 1,
            sample_set: SampleSet::Normal,
            stack_leniency: 0.7,
            mode: Mode::Osu,
            letter_box_in_breaks: false,
            use_skin_sprites: false,
            overlay_position: OverlayPosition::NoChange,
            skin_preference: String::new(),
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
        }
    }
}

#[derive(Default)]
pub struct Editor {
    pub bookmarks: Vec<f64>,
    pub distance_spacing: f32,
    pub beat_divisor: f32,
    pub grid_size: f32,
    pub timeline_zoom: f32,
}

#[derive(Default)]
pub struct Metadata {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: Vec<String>,
    pub beatmap_id: i32,
    pub beatmap_set_id: i32,
}

#[derive(Default)]
pub struct Difficulty {
    pub hp_drain_rate: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
    pub slider_multiplier: f32,
    pub slider_tick_rate: f32,
}

#[derive(Default)]
pub struct Events {}

#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);
