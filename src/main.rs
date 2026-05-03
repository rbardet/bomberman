mod window;

fn main() {
    let ctx: window::WindowContext = window::init_window();
    window::show_window(ctx);
}