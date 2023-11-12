use pixels::{Pixels, SurfaceTexture};
// use winit::event::VirtualKeyCode
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let size = LogicalSize::new(640.0, 480.0);
    let window = WindowBuilder::new()
        .with_title("Chaikin Window")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();
    let mut input = WinitInputHelper::new();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(320, 240, surface_texture).unwrap()
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
                }
                _ => (),
            }
        })
        .unwrap();
}
