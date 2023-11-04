use std::ffi::{CStr, CString};

use eyre::Context;
use gl::types::*;
use glutin::prelude::GlDisplay;

use crate::shader::{Program, ProgramBuilder, Shader};

use ::eyre::Result;

use safe_gl::*;

#[derive(Debug)]
pub(crate) struct Renderer<'a> {
    program: Program<'a>,
}

// Safety: explicitly added the null terminator
const VERT: &'static CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(concat!(include_str!("vert.glsl"), "\0").as_bytes())
};
const FRAG: &'static CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(concat!(include_str!("frag.glsl"), "\0").as_bytes())
};

// all
impl<'a> Renderer<'a> {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Result<Self> {
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

        println!("GL version: {}", get_string(gl::VERSION).unwrap());
        println!(
            "GL version: {}",
            get_string(gl::SHADING_LANGUAGE_VERSION).unwrap()
        );

        let v_shader: Shader<{ gl::VERTEX_SHADER }> = Shader::from_bytes(VERT)?;
        let f_shader: Shader<{ gl::FRAGMENT_SHADER }> = Shader::from_bytes(FRAG)?;

        let prog = ProgramBuilder::new()?
            .attach(&v_shader)
            .attach(&f_shader)
            .build()?;

        Ok(Self { program: prog })
    }

    pub fn draw(&self) {
        clear_colour(0.7, 0.6, 0.8, 1.0);
        clear(gl::COLOR_BUFFER_BIT);
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe { gl::Viewport(0, 0, width as GLsizei, height as GLsizei) };
    }
}

/// safe abstractions of opengl functions
mod safe_gl {
    use super::*;

    pub(super) fn get_string(name: GLenum) -> Option<&'static str> {
        // Safety: as it is restricted to renderer gl has been loaded and
        // we handle the null case
        let cstr: *const u8 = unsafe { gl::GetString(name) };
        (!cstr.is_null())
            .then(|| unsafe { CStr::from_ptr(cstr.cast()) })
            .and_then(|cstr| cstr.to_str().ok())
    }

    pub(super) fn clear_colour(r: f32, g: f32, b: f32, a: f32) {
        unsafe { gl::ClearColor(r as GLfloat, g as GLfloat, b as GLfloat, a as GLfloat) };
    }

    pub(super) fn clear(bits: GLbitfield) {
        unsafe { gl::Clear(bits) };
    }
}
