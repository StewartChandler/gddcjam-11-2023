use std::ffi::c_void;

use gl::types::*;

use crate::mesh::Mesh;

#[derive(Debug)]
pub struct VertArrayObject {
    inner: GLuint,
    pub vbo: VertBufObject,
}

impl VertArrayObject {
    pub fn new(mesh: &Mesh) -> Self {
        let mut vao: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao as *mut GLuint) };
        Self {
            inner: vao,
            vbo: VertBufObject::from_mesh(mesh),
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.inner) };
    }
}

impl Drop for VertArrayObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.inner as *const GLuint) };
    }
}

#[derive(Debug)]
pub struct VertBufObject {
    inner: GLuint,
}

impl VertBufObject {
    fn from_mesh(mesh: &Mesh) -> Self {
        let mut vbo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut vbo as *mut GLuint) };
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo) };
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.buf.len() * std::mem::size_of::<[GLfloat; 8]>()) as GLsizeiptr,
                mesh.buf.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            )
        };

        Self { inner: vbo }
    }
}

impl Drop for VertBufObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.inner as *const GLuint) };
    }
}
