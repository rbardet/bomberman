use std::{fs, path::PathBuf};

use sdl2::keyboard::Scancode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoSettings
{
    width: u32,
    height: u32,
    fullscreen: bool,
}

impl  VideoSettings
{
    pub fn width(&self) -> &u32{ &self.width }
    pub fn height(&self) -> &u32 { &self.height }
    pub fn is_fullscreen(&self) -> &bool { &self.fullscreen }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindings
{
    move_up: u32,
    move_down: u32,
    move_left: u32,
    move_right: u32,
    place_bomb: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings
{
    #[serde(rename = "VideoSettings")]
    pub video: VideoSettings,

    #[serde(rename = "PlayerSettings")]
    pub players: Vec<KeyBindings>,
}

const GAME_NAME: &'static str = "Bomberman";
const CONFIG_FILE: &'static str = "settings.json";

impl Settings
{
    fn new() -> Settings
    {
        Settings::init_default_settings()
    }

    pub fn init() -> Result<Settings, String>
    {
        let path = Settings::get_settings_file_path();

        if path.exists()
        {
            Ok( Settings::load_settings() )
        }
        else
        {
            let settings = Settings::new();
            Settings::save_settings(&settings);
            Ok( settings )
        }
    }

    fn get_settings_file_path() -> PathBuf
    {
        let mut path = dirs::data_local_dir().expect("Failed to to find settings path");
        path.push(GAME_NAME);
        path.push(CONFIG_FILE);
        path
    }

    fn save_settings(settings: &Settings)
    {
        let path = Settings::get_settings_file_path();

        if let Some(parent) = path.parent()
        {
            fs::create_dir_all(parent).expect("Failed to create config folder")
        }

        let json: String = serde_json::to_string_pretty(settings).expect("Failed to save settings");

        fs::write(Settings::get_settings_file_path(), json).expect("Failed to save settings");
    }

    fn load_settings() -> Settings
    {
        let json: String =
            fs::read_to_string(Settings::get_settings_file_path()).expect("Failed to load settings");

        let settings: Settings = serde_json::from_str(&json).expect("Failed to load settings");

        settings
    }

    fn init_default_settings() -> Settings
    {
        Settings
        {
            video: VideoSettings
            {
                width: 800,
                height: 600,
                fullscreen: false,
            },
            players: vec!
            [
                KeyBindings
                {
                    move_up: Scancode::W as u32,
                    move_down: Scancode::S as u32,
                    move_left: Scancode::A as u32,
                    move_right: Scancode::D as u32,
                    place_bomb: Scancode::Space as u32,
                },
                KeyBindings
                {
                    move_up: Scancode::Up as u32,
                    move_down: Scancode::Down as u32,
                    move_left: Scancode::Left as u32,
                    move_right: Scancode::Right as u32,
                    place_bomb: Scancode::LCtrl as u32,
                },
                KeyBindings
                {
                    move_up: Scancode::Kp8 as u32,
                    move_down: Scancode::Kp5 as u32,
                    move_left: Scancode::Kp4 as u32,
                    move_right: Scancode::Kp6 as u32,
                    place_bomb: Scancode::Kp0 as u32,
                },
                KeyBindings
                {
                    move_up: Scancode::I as u32,
                    move_down: Scancode::K as u32,
                    move_left: Scancode::J as u32,
                    move_right: Scancode::L as u32,
                    place_bomb: Scancode::U as u32,
                },
            ],
        }
    }
}