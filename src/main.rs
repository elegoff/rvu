use clap::Parser;
use thiserror::Error;
use winit::{
    error::OsError,
    event::{ElementState, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Debug, Error)]
enum RvuError {
    #[error("Unable to create window")]
    WindowError(#[from] OsError),
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Config {
    /// Name of the image to view
    file_name: String,
}

type Result<T> = std::result::Result<T, RvuError>;

fn main() -> Result<()> {
    let config = Config::parse();
    println!("The filename is {}", config.file_name);

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("My image viewer")
        .build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent { window_id, event } if window_id == window.id() => {
                match event {
                    winit::event::WindowEvent::Resized(_) => (),

                    winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    winit::event::WindowEvent::ScaleFactorChanged {
                        scale_factor: _,
                        new_inner_size: _,
                    } => todo!(),
                    _ => (),
                }
            }
            winit::event::Event::RedrawRequested(_) => (),
            _ => (),
        }
    });
}
