use sdl2::{
    AudioSubsystem,
    Sdl,
    mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS}
};

pub struct AudioContext
{
    audio: AudioSubsystem,
}

const FREQUENCY: i32 = 44_100;
const FORMAT: u16 = AUDIO_S16LSB;
const CHANNELS: i32 = DEFAULT_CHANNELS;
const CHUNK_SIZE: i32 = 1_024;

const NUM_CHANNELS: i32 = 4;

impl AudioContext {
    pub fn new(sdl: &Sdl) -> Result<AudioContext, String>
    {
        sdl2::mixer::open_audio(
            FREQUENCY,
            FORMAT,
            CHANNELS, 
            CHUNK_SIZE
        ).unwrap();

        sdl2::mixer::allocate_channels(NUM_CHANNELS);

        let audio = sdl.audio().unwrap();

        Ok( AudioContext { audio })
    }

    pub fn audio(&self) -> &AudioSubsystem { &self.audio }

    // fn play_music(music_file: &Path) 
    // {
    //     let music = sdl2::mixer::Music::from_file(music_file).unwrap();

    //     music.play(1);
    // }
}
