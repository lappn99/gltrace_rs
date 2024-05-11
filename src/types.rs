use core::ffi::{*};


//Types for the hook generator corresponding to OpenGL types
//https://www.khronos.org/opengl/wiki/OpenGL_Type
pub mod types {
    pub type GLboolean = super::c_uchar;
    pub type GLbyte = super::c_uchar;
    pub type GLshort = super::c_short;
    pub type GLushort = super::c_ushort;
    pub type GLint = super::c_int;
    pub type GLuint = super::c_uint;
    pub type GLfixed = super::c_int;
    pub type GLint64 = super::c_long;
    pub type GLuint64 = super::c_ulong;
    pub type GLsizei = super::c_int;
    pub type GLenum = super::c_int;
    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;
    pub type GLsync = *const GLSyncHandle;
    pub type GLbitfield = super::c_int;
    pub type GLhlaf = super::c_ushort;
    pub type GLfloat = super::c_float;
    pub type GLclampf = super::c_float;
    pub type GLdouble = super::c_double;
    pub type GLclampd = super::c_double;
    pub type GLchar = super::c_char;
    pub type GLubyte = super::c_uchar;

    pub enum GLSyncHandle {}

    pub type GLDEBUGPROC = Option<
        extern "system" fn(
            source: GLenum,
            gltype: GLenum,
            id: GLuint,
            severity: GLenum,
            length: GLsizei,
            message: *const GLchar,
            userParam: *mut super::c_void,
        ),
    >;
    pub type GLDEBUGPROCARB = Option<
        extern "system" fn(
            source: GLenum,
            gltype: GLenum,
            id: GLuint,
            severity: GLenum,
            length: GLsizei,
            message: *const GLchar,
            userParam: *mut super::c_void,
        ),
    >;
    pub type GLDEBUGPROCKHR = Option<
        extern "system" fn(
            source: GLenum,
            gltype: GLenum,
            id: GLuint,
            severity: GLenum,
            length: GLsizei,
            message: *const GLchar,
            userParam: *mut super::c_void,
        ),
    >;
}
