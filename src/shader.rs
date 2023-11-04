use std::{ffi::CStr, marker::PhantomData, num::NonZeroU32, ops::Deref, ptr};

use gl::types::*;

use eyre::{eyre, Context, Result};

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

impl<'a, const ST: GLenum> Shader<'a, ST> {
    pub fn from_bytes<'b>(source: &'b CStr) -> Result<Self> {
        // let source_cstr = CStr::from_bytes_with_nul(source)?;
        let res = Self {
            inner: NonZeroU32::new(unsafe { gl::CreateShader(ST) })
                .ok_or_else(|| eyre!("unable to create shader of type {}", ST))?,
            _pd: PhantomData {},
        };
        unsafe {
            gl::ShaderSource(
                *res,
                1,
                (&source.as_ptr()) as *const *const GLchar,
                ptr::null_mut(),
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

pub(crate) struct ProgramBuilder<'a> {
    inner: NonZeroU32,
    _pd: PhantomData<&'a ()>,
}

impl ProgramBuilder<'_> {
    fn get<'a>(&'a self) -> &'a GLuint {
        unsafe {
            ((&self.inner as *const NonZeroU32) as *const u32)
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

impl<'pb> ProgramBuilder<'pb> {
    pub fn new() -> Result<Self>
    {
        let program: GLuint = unsafe { gl::CreateProgram() };
        Ok(ProgramBuilder {
            inner: NonZeroU32::new(program).ok_or_else(|| eyre!("could not create the program"))?,
            _pd: PhantomData {},
        })
    }

    pub fn attach<'b, const ST: GLenum>(self, shader: &'b Shader<ST>) -> Self
    where
        'b: 'pb,
    {
        unsafe { gl::AttachShader(*self.get(), **shader) };
        self
    }

    pub fn gl_bind_attrib(self, index: GLuint, name: &CStr) -> Self {
        unsafe { gl::BindAttribLocation(*self.get(), index, name.as_ptr()) };
        self
    }

    pub fn build<'a>(self) -> Result<Program<'a>> 
    where
        'a: 'pb
    {
        unsafe { gl::LinkProgram(*self.get()) };

        let status = {
            let mut status: GLint = 0;
            unsafe { gl::GetProgramiv(*self.get(), gl::LINK_STATUS, &mut status as *mut GLint) };
            status as GLboolean
        };
        if status == gl::FALSE {
            let mut log_len: GLint = 0;
            unsafe {
                gl::GetProgramiv(*self.get(), gl::INFO_LOG_LENGTH, &mut log_len as *mut GLint)
            };
            let mut buf = vec![0u8; log_len as usize].into_boxed_slice();
            unsafe {
                gl::GetProgramInfoLog(
                    *self.get(),
                    log_len as GLsizei,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                )
            }

            let err_str: &str = CStr::from_bytes_until_nul(buf.as_mut())
                .wrap_err("program info log not null terminated")?
                .to_str()
                .wrap_err("program info log is not valid utf-8")?;

            return Err(eyre!(
                "program failed to link with info log: `{}`",
                err_str
            ));
        }

        Ok(Program {
            inner: self.inner,
            _pd: PhantomData {},
        })
    }
}

#[derive(Debug)]
pub(crate) struct Program<'a> {
    inner: NonZeroU32,
    _pd: PhantomData<&'a ()>,
}

impl Program<'_> {
    pub fn use_program(&mut self) {
        unsafe { gl::UseProgram(**self) };
    }
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
