use eyre::{WrapErr, Result};
use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent},
    window::WindowBuilder
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new().wrap_err("failed to initialize the event loop")?;
    let window = WindowBuilder::new().build(&event_loop).wrap_err("failed to build window")?;

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit();
            },
            Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        }
    })?;

    Ok(())
}
