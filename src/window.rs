use sdl2::{self, event::Event, keyboard::Keycode};

use crate::settings::Settings;

pub struct SDLWindowContext {
    sdl_ctx: sdl2::Sdl,
	opengl_ctx: sdl2::video::GLContext,
    window_ctx: sdl2::video::Window,
}

const WINDOW_TITLE: &'static str = "Bomberman v0.1";

pub fn init_window(settings: Settings) -> SDLWindowContext {
    let sdl_ctx: sdl2::Sdl = sdl2::init().expect("SDL: Failed on init.");
    let video_subsystem: sdl2::VideoSubsystem = sdl_ctx.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_stencil_size(8);

    let window_ctx: sdl2::video::Window = video_subsystem
        .window(
            WINDOW_TITLE,
            settings.video.width,
            settings.video.height,
        )
        .opengl()
        .build()
        .expect("SDL: Failed to create window.");

    let opengl_ctx: sdl2::video::GLContext = window_ctx.gl_create_context().unwrap();
    gl::load_with(|s: &'static str| video_subsystem.gl_get_proc_address(s) as *const _);

    SDLWindowContext {
        sdl_ctx,
		opengl_ctx,
        window_ctx
    }
}

pub fn run_window(ctx: &mut SDLWindowContext) {
    let mut event_pump: sdl2::EventPump = ctx.sdl_ctx.event_pump().unwrap();
    let mut is_running: bool = true;

    loop {
        for event in event_pump.poll_iter() {
            sdl_handle_event(&mut ctx.window_ctx, event, &mut is_running);
        }

        if !is_running {
            break;
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        ctx.window_ctx.gl_swap_window();
    }
}

fn sdl_handle_event(
    window: &mut sdl2::video::Window,
    event: sdl2::event::Event,
    is_running: &mut bool,
) {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            *is_running = false;
        }
        // Handle other events here
        _ => {}
    }
}
