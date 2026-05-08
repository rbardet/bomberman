use std::{fs, path::PathBuf};

use sdl2::keyboard::Scancode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoSettings {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindings {
    pub move_up: u32,
    pub move_down: u32,
    pub move_left: u32,
    pub move_right: u32,
    pub place_bomb: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    #[serde(rename = "VideoSettings")]
    pub video: VideoSettings,

    #[serde(rename = "PlayerSettings")]
    pub players: Vec<KeyBindings>,
}

const GAME_NAME: &'static str = "Bomberman";
const CONFIG_FILE: &'static str = "settings.json";

fn get_settings_file_path() -> PathBuf {
    let mut path = dirs::data_local_dir().expect("Failed to to find settings path");
    path.push(GAME_NAME);
    path.push(CONFIG_FILE);
    path
}

pub fn save_settings(settings: &Settings) {
    let path = get_settings_file_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create config folder")
    }

    let json: String = serde_json::to_string_pretty(settings).expect("Failed to save settings");

    fs::write(get_settings_file_path(), json).expect("Failed to save settings");
}

pub fn load_settings() -> Settings {
    let json: String =
        fs::read_to_string(get_settings_file_path()).expect("Failed to load settings");

    let settings: Settings = serde_json::from_str(&json).expect("Failed to load settings");

    settings
}

pub fn init_settings() -> Settings {
    let config_file: PathBuf = get_settings_file_path();

    if config_file.exists() {
        load_settings()
    } else {
        let settings: Settings = init_default_settings();
        save_settings(&settings);
        settings
    }
}

fn init_default_settings() -> Settings {
    Settings {
        video: VideoSettings {
            width: 800,
            height: 600,
            fullscreen: false,
        },
        players: vec![
            KeyBindings {
                move_up: Scancode::W as u32,
                move_down: Scancode::S as u32,
                move_left: Scancode::A as u32,
                move_right: Scancode::D as u32,
                place_bomb: Scancode::Space as u32,
            },
            KeyBindings {
                move_up: Scancode::Up as u32,
                move_down: Scancode::Down as u32,
                move_left: Scancode::Left as u32,
                move_right: Scancode::Right as u32,
                place_bomb: Scancode::LCtrl as u32,
            },
            KeyBindings {
                move_up: Scancode::Kp8 as u32,
                move_down: Scancode::Kp5 as u32,
                move_left: Scancode::Kp4 as u32,
                move_right: Scancode::Kp6 as u32,
                place_bomb: Scancode::Kp0 as u32,
            },
            KeyBindings {
                move_up: Scancode::I as u32,
                move_down: Scancode::K as u32,
                move_left: Scancode::J as u32,
                move_right: Scancode::L as u32,
                place_bomb: Scancode::U as u32,
            },
        ],
    }
}
