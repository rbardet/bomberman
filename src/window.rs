use sdl2::{Sdl, event::Event, keyboard::Keycode};

use crate::settings::Settings;

pub struct WindowContext
{
    opengl: sdl2::video::GLContext,
    window: sdl2::video::Window,
}

const WINDOW_TITLE: &'static str = "Bomberman v0.1";

impl WindowContext
{   
    pub fn init(sdl: &Sdl, settings: &Settings) -> Result<WindowContext, String>
    {
        WindowContext::new(sdl, settings)
    }

    fn new(sdl: &Sdl, settings: &Settings) -> Result<WindowContext, String>
    {
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_stencil_size(8);

        let window = video_subsystem
            .window(
                WINDOW_TITLE,
                settings.video.width().clone(),
                settings.video.height().clone()
            )
            .opengl()
            .build()
            .expect("SDL: Failed to create window.");

        let opengl = window.gl_create_context().unwrap();
        gl::load_with(|s: &'static str| video_subsystem.gl_get_proc_address(s) as *const _);

        Ok(WindowContext{
            opengl,
            window,
        })
    }

    pub fn opengl(&self) -> &sdl2::video::GLContext { &self.opengl }
    pub fn window(&self) -> &sdl2::video::Window { &self.window }

    pub fn run_window(&self, sdl: &Sdl)
    {
        let mut event_pump = sdl.event_pump().unwrap();
        let mut is_running = true;

        loop
        {
            for event in event_pump.poll_iter()
            {
                match event
                {
                    Event::Quit { .. }
                    | Event::KeyDown
                    {
                        keycode: Some(Keycode::Escape),
                        ..
                    } =>
                    {
                        is_running = false;
                    }
                    // Handle other events here
                    _ => {}
                }
            }

            if !is_running
            {
                break;
            }

            unsafe
            {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            &self.window.gl_swap_window();
        }
    }
}
