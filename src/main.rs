use std::num::NonZeroU32;

use eyre::{eyre, Result, WrapErr};

use raw_window_handle::HasRawWindowHandle;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::{GlConfig, GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::GlSurface,
};

use glutin_winit::{finalize_window, DisplayBuilder, GlWindow};

mod renderer;
mod shader;

use renderer::Renderer;

fn main() -> Result<()> {
    let event_loop = EventLoop::new().wrap_err("failed to initialize the event loop")?;
    let window_builder = WindowBuilder::new();

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let template_builder = ConfigTemplateBuilder::new();

    let (mut window, gl_config) = display_builder
        .build(&event_loop, template_builder, |configs| {
            configs.max_by_key(|config| config.num_samples()).unwrap()
        })
        .map_err(|e| eyre!("could not make display: {}", e))?;

    let raw_window_handle = window.as_ref().map(|w| w.raw_window_handle());

    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    let gl_display = gl_config.display();

    // saftey: as we created `raw_window_handle` just before this, it points to
    // a valid window object
    let mut inactive_context = Some(
        unsafe { gl_display.create_context(&gl_config, &context_attributes) }
            .wrap_err("failed to crate context")?,
    );

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = None;
    let mut renderer = None;
    event_loop.run(move |event, elwt| match event {
        // code to initialize gl_context and make current should be in resuemed
        // acording to docs
        Event::Resumed => {
            let window = window
                .take()
                .map(|w| Ok(w))
                .unwrap_or_else(|| {
                    let window_builder = WindowBuilder::new();
                    finalize_window(elwt, window_builder, &gl_config)
                })
                .unwrap();

            let attrs = window.build_surface_attributes(<_>::default());
            let gl_surface =
                unsafe { gl_display.create_window_surface(&gl_config, &attrs) }.unwrap();

            // make context current
            let gl_context = inactive_context
                .take()
                .unwrap()
                .make_current(&gl_surface)
                .unwrap();

            // initialize renderer
            renderer.get_or_insert_with(|| Renderer::new(&gl_display).unwrap());

            state = Some((gl_context, gl_surface, window));
        }
        Event::Suspended => {
            let (gl_context, ..) = state.take().unwrap();
            assert!(inactive_context
                .replace(gl_context.make_not_current().unwrap())
                .is_none());
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                if size.width != 0 && size.height != 0 {
                    if let Some((gl_context, gl_surface, _)) = &state {
                        gl_surface.resize(
                            gl_context,
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        );

                        renderer.as_ref().unwrap().resize(size.width, size.height);
                    }
                }
            }
            WindowEvent::CloseRequested => {
                elwt.exit();
            }
            _ => (),
        },
        Event::AboutToWait => {
            if let Some((gl_context, gl_surface, window)) = &state {
                renderer.as_ref().unwrap().draw();

                window.request_redraw();

                gl_surface.swap_buffers(gl_context).unwrap();
            }
        }
        _ => (),
    })?;

    Ok(())
}
