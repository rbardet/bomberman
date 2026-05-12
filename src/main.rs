use crate::engine::Engine;

mod audio;
mod settings;
mod window;
mod engine;

fn main()
{
    let engine = Engine::init().unwrap();
    engine.win_ctx().run_window(engine.sdl());
}
