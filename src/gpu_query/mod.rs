use crate::hooks::gl;

use crate::types::types::{GLboolean, GLenum, GLint, GLint64, GLuint, GLuint64};
pub mod enums;
pub mod errors;


use self::enums::QueryTarget;

use super::Result;

pub struct QueryObject {
    name: Option<GLuint>,
}

pub trait QueryResult<T> {
    fn query_result(&self) -> super::Result<T>;

}

impl QueryObject {
    pub fn new() -> Self {
        let mut name: GLuint = 0;
        unsafe {
            let name_ptr: *mut GLuint = &mut name;
            gl::funcs::GenQueries(1, name_ptr);
        }

        Self {
            name: if name > 0 { Some(name) } else { None }
        }
    }

    pub fn begin_query(&self, target: QueryTarget) -> Result<()> {
        if let Some(name) = self.name {
            unsafe {
                let target = GLenum::from(target);
                if target == gl::enums::NONE {
                    Err(errors::GPUQueryError::UnknownTargetError(target).into())
                } else {
                    gl::funcs::BeginQuery(target, name);
                    Ok(())
                }
            }
        } else {
            Err(errors::GPUQueryError::InvalidNameError.into())
        }
    }

    pub fn end_query(&self, target: QueryTarget) -> Result<()> {
        unsafe {
            let target = GLenum::from(target);
            if target == gl::enums::NONE {
                Err(errors::GPUQueryError::UnknownTargetError(target).into())
            } else {
                gl::funcs::EndQuery(target);
                Ok(())
            }
        }
    }

    pub fn query_result_available(&self) -> Result<bool> {
        if let Some(name) = self.name {
            unsafe {
                let mut result = 0;
                let result_ptr: *mut i32 = &mut result;
                gl::funcs::GetQueryObjectiv(name, gl::enums::QUERY_RESULT_AVAILABLE, result_ptr);
                match result as GLboolean {
                    gl::enums::TRUE => Ok(true),
                    gl::enums::FALSE => Ok(false),
                    _ => Err(errors::GPUQueryError::QueryResultError.into()),
                }
            }
        } else {
            Err(errors::GPUQueryError::InvalidNameError.into())
        }
    }
}

macro_rules! query_result_impl {
    ($result_type: ty, $func: tt) => {
        impl QueryResult<$result_type> for QueryObject {
            fn query_result(&self) -> super::Result<$result_type> {
                if let Some(name) = self.name {
                    let mut result: $result_type = Default::default();
                    unsafe {
                        let result_ptr: *mut $result_type = &mut result;
                        gl::funcs::$func(name, gl::enums::QUERY_RESULT, result_ptr);
                    }
                    Ok(result)
                } else {
                    Err(errors::GPUQueryError::InvalidNameError.into())
                }
            }
        }
    };
}

query_result_impl!(GLuint, GetQueryObjectuiv);
query_result_impl!(GLint, GetQueryObjectiv);
query_result_impl!(GLint64, GetQueryObjecti64v);
query_result_impl!(GLuint64, GetQueryObjectui64v);
