use std::{ffi::CStr, marker::PhantomData, num::NonZeroU32, ops::Deref};

use gl::types::*;

use eyre::{eyre, Context, Result};

use crate::renderer::Renderer;

#[derive(Debug)]
pub(crate) struct Shader<'a, const ST: GLenum> {
    inner: NonZeroU32,
    _pd: PhantomData<&'a ()>,
}

impl<'a, const ST: GLenum> Deref for Shader<'a, ST> {
    type Target = GLuint;

    fn deref<'b>(&'b self) -> &'b Self::Target {
        // Safety: a nonzero u32 is itself a u32 so the conversion is valid
        // as it is imutable, it's fine
        unsafe {
            ((&self.inner as *const NonZeroU32) as *const u32)
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

impl<'a, const ST: GLenum> Drop for Shader<'a, ST> {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.inner.into()) }
    }
}

impl<const ST: GLenum> Shader<'_, ST> {
    pub fn from_bytes<'a>(_: &'a Renderer, source: &'static [u8]) -> Result<Self> {
        // let source_cstr = CStr::from_bytes_with_nul(source)?;
        let res = Self {
            inner: NonZeroU32::new(unsafe { gl::CreateShader(ST) })
                .ok_or_else(|| eyre!("unable to create shader of type {}", ST))?,
            _pd: PhantomData {},
        };
        let len: GLint = source.len() as GLint;
        unsafe {
            gl::ShaderSource(
                *res,
                1,
                (&(source.as_ptr()) as *const *const u8) as *const *const GLchar,
                &len as *const GLint,
            )
        }

        unsafe { gl::CompileShader(*res) };

        let mut len: GLint = 0;
        unsafe { gl::GetShaderiv(*res, gl::INFO_LOG_LENGTH, &mut len as *mut GLint) };

        if len > 0 {
            let mut buf = vec![0u8; len as usize].into_boxed_slice();
            let mut len_written: GLsizei = 0;
            unsafe {
                gl::GetShaderInfoLog(
                    *res,
                    len,
                    &mut len_written as *mut GLsizei,
                    buf.as_mut_ptr() as *mut GLchar,
                )
            };

            let err_str: &str = CStr::from_bytes_until_nul(buf.as_mut())
                .wrap_err("shader info log not null terminated")?
                .to_str()
                .wrap_err("shader info log is not valid utf-8")?;

            return Err(eyre!(
                "shader failed to compile with info log: `{}`",
                err_str
            ));
        }

        Ok(res)
    }
}

#[derive(Debug)]
pub(crate) struct Program<'a> {
    inner: NonZeroU32,
    _pd: PhantomData<&'a ()>,
}

impl Program<'_> {
    // TODO
}

impl<'a> Deref for Program<'a> {
    type Target = GLuint;

    fn deref(&self) -> &Self::Target {
        // Safety: a nonnull u32 is itself a u32 so the conversion is valid
        // as it is imutable, it's fine
        unsafe {
            ((&self.inner as *const NonZeroU32) as *const u32)
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

impl<'a> Drop for Program<'a> {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.inner.into()) };
    }
}

/// safe abstractions of opengl functions
mod safe_gl {
    use super::*;
}
