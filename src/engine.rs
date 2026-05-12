use sdl2::Sdl;

use crate::{
    audio::AudioContext,
    settings::Settings,
    window::WindowContext
};

pub struct Engine
{
    sdl: Sdl,
    win_ctx: WindowContext,
    audio_ctx: AudioContext,
    settings: Settings,
}

impl Engine
{   
    pub fn init() -> Result<Engine, String>
    {
        Engine::new()
    }

    pub fn new() -> Result<Engine, String>
    {   
        let settings = Settings::init()?;
        let sdl  = sdl2::init()?;
        let win_ctx = WindowContext::init(&sdl, &settings)?;
        let audio_ctx = AudioContext::new(&sdl)?;

        Ok(Engine{
            sdl,
            win_ctx,
            audio_ctx,
            settings,
        })
    }

    pub fn sdl(&self) -> &Sdl { &self.sdl }
    pub fn win_ctx(&self) -> &WindowContext { &self.win_ctx }
    pub fn audio_ctx(&self) -> &AudioContext { &self.audio_ctx }
    pub fn settings(&self) -> &Settings { &self.settings }
}