use beryllium::{Sdl, events, init, video};

use crate::settings::Settings;

pub struct WindowConfig {
    title: &'static str,
    allow_high_dpi: bool,
    borderless: bool,
    resizable: bool,
}

pub struct WindowContext {
    sdl: Sdl,
    win: video::GlWindow,
}

const WINDOW_CONFIG: WindowConfig = WindowConfig {
    title: "Bomberman",
    allow_high_dpi: true,
    borderless: false,
    resizable: true,
};

pub fn init_window(settings: Settings) -> WindowContext {
    let sdl: Sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3)
        .expect("failed to set OpenGL version");
    sdl.set_gl_profile(video::GlProfile::Core)
        .expect("failed to set OpenGL Core profile");

    let win_args: video::CreateWinArgs<'_> = video::CreateWinArgs {
        title: WINDOW_CONFIG.title,
        width: settings.video.width as i32,
        height: settings.video.height as i32,
        allow_high_dpi: WINDOW_CONFIG.allow_high_dpi,
        borderless: WINDOW_CONFIG.borderless,
        resizable: WINDOW_CONFIG.resizable,
    };

    let win: video::GlWindow = sdl
        .create_gl_window(win_args)
        .expect("window creation failed");

    WindowContext { sdl, win }
}

pub fn show_window(ctx: WindowContext) {
    'main_loop: loop {
        while let Some(event) = ctx.sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }
        ctx.win.swap_window();
    }
}
