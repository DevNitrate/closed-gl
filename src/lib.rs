mod functions;

pub type GLenum = u32;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLint = i32;

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

#[cfg(test)]
mod tests {
    

    #[test]
    fn testing() {

        assert_eq!(result, 4);
    }
}
