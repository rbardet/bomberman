use crate::{
    settings::{Settings, init_settings},
    window::{SDLWindowContext, init_window, run_window},
};

mod settings;
mod window;

fn main() {
    let settings: Settings = init_settings();
    let mut ctx: SDLWindowContext = init_window(settings);
    run_window(&mut ctx);
}
