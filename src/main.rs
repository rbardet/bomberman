use crate::settings::{Settings, init_settings};

mod window;
mod settings;

fn main() {
    let settings: Settings = init_settings();
    let ctx: window::WindowContext = window::init_window(settings);
    window::show_window(ctx);
}