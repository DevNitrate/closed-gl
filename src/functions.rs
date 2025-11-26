use std::ffi::c_void;

use crate::{GLbitfield, GLenum, GLerror, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint, constants::GL_FALSE};

pub fn gl_get_error() -> GLenum {
    unsafe { gl::GetError() }
}

pub fn gl_load_with<F: FnMut(&'static str) -> *const c_void>(loadfn: F) {
    gl::load_with(loadfn);
}

pub fn check_errors<T>(output: T) -> Result<T, GLerror> {
    let err: GLenum = gl_get_error();

    if err == 0 {
        return Ok(output);
    } else {
        Err(GLerror::from_gl_error(err))
    }    
}

pub fn gl_create_shader(shader_type: GLenum) -> Result<GLuint, GLerror> {
    let output: u32 = unsafe { gl::CreateShader(shader_type) };

    if output == 0 {
       return Err(GLerror::CreateShaderFail);
    }

    return Ok(output);
}

pub fn gl_shader_source(shader: GLuint, count: GLsizei, string: &str, length: &mut GLint) {
    let str_ptr: [*const i8; 1] = [string.as_ptr() as *const i8];
    unsafe { gl::ShaderSource(shader, count, str_ptr.as_ptr(), length); };
}

pub fn gl_compile_shader(shader: GLuint) {
    unsafe { gl::CompileShader(shader) };
}

pub fn gl_create_program() -> Result<GLuint, GLerror> {
    let output: GLuint = unsafe { gl::CreateProgram() };

    if output == 0 {
        return Err(GLerror::CreateProgramFail);
    }

    return Ok(output);
}

pub fn gl_attach_shader(program: GLuint, shader: GLuint) {
    unsafe { gl::AttachShader(program, shader) };
}

pub fn gl_link_program(program: GLuint) {
    unsafe { gl::LinkProgram(program) };
}

pub fn gl_delete_shader(shader: GLuint) {
    unsafe { gl::DeleteShader(shader) };
}

pub fn gl_use_program(program: GLuint) {
    unsafe { gl::UseProgram(program) };
}

pub fn gl_dispatch_compute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint, ) {
    unsafe { gl::DispatchCompute(num_groups_x, num_groups_y, num_groups_z) };
}

pub fn gl_gen_buffers<T>(n: GLsizei, buffers: &mut [GLuint]) {
    unsafe { gl::GenBuffers(n, buffers.as_mut_ptr()); }
}

pub fn gl_bind_buffer(target: GLenum, buffer: GLuint) {
    unsafe { gl::BindBuffer(target, buffer); }
}

pub fn gl_buffer_data<T>(target: GLenum, data: &[T], usage: GLenum) {
    unsafe { gl::BufferData(target, (data.len() * size_of::<T>()) as GLsizeiptr, data.as_ptr() as *const c_void, usage); }
}

pub fn gl_bind_buffer_base(target: GLenum, index: GLuint, buffer: GLuint) {
    unsafe { gl::BindBufferBase(target, index, buffer); }
}

pub fn gl_memory_barrier(barriers: GLbitfield) {
    unsafe { gl::MemoryBarrier(barriers); }
}

pub fn gl_unmap_buffer(target: GLenum) -> Result<(), GLerror> {
    let output = unsafe { gl::UnmapBuffer(target) };

    if output == GL_FALSE {
        check_errors(())
    } else {
        Ok(())
    }
}

pub fn gl_delete_program(program: GLuint) {
    unsafe { gl::DeleteProgram(program); }
}

pub fn gl_delete_buffers(n: GLsizei, buffers: &mut [GLuint]) {
    unsafe { gl::DeleteBuffers(n, buffers.as_mut_ptr()); }
}

pub fn gl_map_buffer_range<T>(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> Vec<T> {
    let ptr: *mut T = unsafe { gl::MapBufferRange(target, offset, length, access)  as *mut T };

    unsafe { Vec::from_raw_parts(ptr, length as usize, (length as usize) * size_of::<T>()) }
}