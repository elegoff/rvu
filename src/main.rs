use std::cmp::max;

use clap::Parser;
use thiserror::Error;
use winit::{
    dpi::PhysicalSize,
    error::OsError,
    event::{ElementState, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Debug, Error)]
enum RvuError {
    #[error("Unable to create window")]
    WindowError(#[from] OsError),
    #[error("Failed to process image")]
    ImageError(#[from] image::ImageError),
    #[error("Failed to load image")]
    IOError(#[from] std::io::Error),
    #[error("Unable to calculate screen size")]
    NoPrimaryMonitor,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Config {
    /// Name of the image to view
    file_name: String,
}

type Result<T> = std::result::Result<T, RvuError>;

const SCREEN_PERCENT: u32 = 90;

fn main() -> Result<()> {
    let config = Config::parse();

    let image = image::io::Reader::open(config.file_name)?.decode()?;

    let w = image.width();
    let h = image.height();

    println!("image is {} by {}", w, h);

    let event_loop = EventLoop::new();
    let primary_monitor = event_loop
        .primary_monitor()
        .ok_or(RvuError::NoPrimaryMonitor)?;

    let screen_size = primary_monitor.size();
    let max_screen_size = (
        screen_size.width * SCREEN_PERCENT / 100,
        screen_size.height * SCREEN_PERCENT / 100,
    );
    let horz_scale = calc_scale(max_screen_size.0, image.width());
    let vert_scale = calc_scale(max_screen_size.1, image.height());
    let scale = max(horz_scale, vert_scale);

    let window_inner_size = PhysicalSize::new(image.width() / scale, image.height() / scale);

    let window = WindowBuilder::new()
        .with_title("My image viewer")
        .with_inner_size(window_inner_size)
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

fn calc_scale(max_size: u32, cur_size: u32) -> u32 {
    if max_size > cur_size {
        1
    } else {
        ((cur_size as f32) / (max_size as f32)).ceil() as u32
    }
}
