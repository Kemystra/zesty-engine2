use winit::event_loop::{ControlFlow, EventLoop};
use clap::Parser;

mod app;
mod scene;
mod renderer;
mod math_utils;
mod transform;

use app::App;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    scene_filename: String
}

fn main() {
    let args = Arguments::parse();
    let mut app = App::new(args);

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}
