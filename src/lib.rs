pub mod functions;
pub mod constants;

pub type GLenum = u32;
pub type GLuint = u32;
pub type GLbuffer = u32;
pub type GLuint64 = u64;
pub type GLsizei = i32;
pub type GLint = i32;
pub type GLboolean = u8;
pub type GLsizeiptr = isize;
pub type GLintptr = isize;
pub type GLbitfield = u32;

pub enum GLerror {
    GlNoError,
    GlInvalidEnum,
    GlInvalidValue,
    GlInvalidOperation,
    GlInvalidFramebufferOperation,
    GlOutOfMemory,
    GlStackUnderflow,
    GlStackOverflow,
    GlUnknown(GLenum),

    // custom errors
    CreateShaderFail,
    CreateProgramFail
}

impl GLerror {
    pub fn from_gl_error(code: GLenum) -> Self {
        match code {
            gl::NO_ERROR => GLerror::GlNoError,
            gl::INVALID_ENUM => GLerror::GlInvalidEnum,
            gl::INVALID_VALUE => GLerror::GlInvalidValue,
            gl::INVALID_OPERATION => GLerror::GlInvalidOperation,
            gl::INVALID_FRAMEBUFFER_OPERATION => GLerror::GlInvalidFramebufferOperation,
            gl::OUT_OF_MEMORY => GLerror::GlOutOfMemory,
            gl::STACK_UNDERFLOW => GLerror::GlStackUnderflow,
            gl::STACK_OVERFLOW => GLerror::GlStackOverflow,
            other => GLerror::GlUnknown(other),
        }
    }
}
