use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
#[allow(unused)]
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use chaikin::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    // Create the Pixels object
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture).unwrap()
    };

    // the following is another way to build a window that is customizable
    // let builder = WindowBuilder::new();
    // builder.build(&event_loop).unwrap();
    // You can create and handle your own custom Event::UserEvents
    event_loop
        .run(move |event, elwt| {
            // if let Event::RedrawRequested(_) = event {
            //     draw(&b, pixels.get_frame());
            //     if pixels.render().is_err() {
            //         control_flow.exit();
            //         return;
            //     }
            // }
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    elwt.exit();
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { state, button, .. },
                    ..
                } => {
                    // Check if the left mouse button was pressed
                    if state == ElementState::Pressed && button == MouseButton::Left {
                        // Call your onclick function here
                        println!("Left mouse button clicked");
                    }
                }
                Event::AboutToWait => {
                    // Application update code.

                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw, in
                    // applications which do not always need to. Applications that redraw continuously
                    // can just render here instead.
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Redraw the application.
                    //
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in AboutToWait, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.
                    // Draw your content here

                    // Clear the frame

                    for pixel in pixels.frame_mut().chunks_exact_mut(4) {
                        pixel.copy_from_slice(&[0, 0, 0, 255]); // Black background
                    }

                    // Draw a white circle at the center
                    let center_x = SCREEN_WIDTH / 2;
                    let center_y = SCREEN_HEIGHT / 2;
                    let radius = 50;
                    for y in center_y - radius..=center_y + radius {
                        for x in center_x - radius..=center_x + radius {
                            if ((x as i32) - (center_x as i32)).pow(2)
                                + ((y as i32) - (center_y as i32)).pow(2)
                                <= radius.pow(2).try_into().unwrap()
                            {
                                let index = ((y * SCREEN_WIDTH + x) as usize) * 4;
                                pixels.frame_mut()[index..index + 3]
                                    .copy_from_slice(&[255, 255, 255]); // White circle
                            }
                        }
                    }

                    // Swap the buffer
                    // Swap the buffer
                    pixels.render().unwrap();
                }
                _ => (),
            }
        })
        .unwrap();
}
