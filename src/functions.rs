use std::ffi::c_void;

use crate::{GLenum, GLerror, GLint, GLsizei, GLuint};

pub fn gl_get_error() -> GLenum {
    unsafe { gl::GetError() }
}

pub fn gl_load_with<F: FnMut(&'static str) -> *const c_void>(loadfn: F) {
    unsafe { gl::load_with(loadfn) };
}

pub fn close_check_errors<T>(output: T) -> Result<T, GLerror> {
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

pub fn gl_shader_source(shader: GLuint, count: GLsizei, string: &'static str, length: &mut GLint) {
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