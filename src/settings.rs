use std::{fs, path::PathBuf};

use beryllium::events::{
    SDL_SCANCODE_A, SDL_SCANCODE_D, SDL_SCANCODE_DOWN, SDL_SCANCODE_I, SDL_SCANCODE_J,
    SDL_SCANCODE_K, SDL_SCANCODE_KP_0, SDL_SCANCODE_KP_4, SDL_SCANCODE_KP_5, SDL_SCANCODE_KP_6,
    SDL_SCANCODE_KP_8, SDL_SCANCODE_L, SDL_SCANCODE_LCTRL, SDL_SCANCODE_LEFT, SDL_SCANCODE_RIGHT,
    SDL_SCANCODE_S, SDL_SCANCODE_SPACE, SDL_SCANCODE_U, SDL_SCANCODE_UP, SDL_SCANCODE_W,
};
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

    let json = serde_json::to_string_pretty(settings).expect("Failed to save settings");

    fs::write(get_settings_file_path(), json).expect("Failed to save settings");
}

pub fn load_settings() -> Settings {
    let json = fs::read_to_string(get_settings_file_path()).expect("Failed to load settings");

    let settings: Settings = serde_json::from_str(&json).expect("Failed to load settings");

    settings
}

pub fn init_settings() -> Settings {
    let config_file = get_settings_file_path();

    if config_file.exists() {
        load_settings()
    } else {
        let settings = init_default_settings();
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
                move_up: SDL_SCANCODE_W.0 as u32,
                move_down: SDL_SCANCODE_S.0 as u32,
                move_left: SDL_SCANCODE_A.0 as u32,
                move_right: SDL_SCANCODE_D.0 as u32,
                place_bomb: SDL_SCANCODE_SPACE.0 as u32,
            },
            KeyBindings {
                move_up: SDL_SCANCODE_UP.0 as u32,
                move_down: SDL_SCANCODE_DOWN.0 as u32,
                move_left: SDL_SCANCODE_LEFT.0 as u32,
                move_right: SDL_SCANCODE_RIGHT.0 as u32,
                place_bomb: SDL_SCANCODE_LCTRL.0 as u32,
            },
            KeyBindings {
                move_up: SDL_SCANCODE_KP_8.0 as u32,
                move_down: SDL_SCANCODE_KP_5.0 as u32,
                move_left: SDL_SCANCODE_KP_4.0 as u32,
                move_right: SDL_SCANCODE_KP_6.0 as u32,
                place_bomb: SDL_SCANCODE_KP_0.0 as u32,
            },
            KeyBindings {
                move_up: SDL_SCANCODE_I.0 as u32,
                move_down: SDL_SCANCODE_K.0 as u32,
                move_left: SDL_SCANCODE_J.0 as u32,
                move_right: SDL_SCANCODE_L.0 as u32,
                place_bomb: SDL_SCANCODE_U.0 as u32,
            },
        ],
    }
}
