use core::fmt;
use std::fmt::write;

use crate::types::types::GLenum;

#[derive(Debug)]
pub enum GPUQueryError {
    UnknownTargetError(GLenum),
    InvalidNameError,
    QueryResultError
}

impl fmt::Display for GPUQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
             GPUQueryError::UnknownTargetError(target) => write!(f,"Unknown target for Query Object: {}", target),
             GPUQueryError::InvalidNameError => write!(f, "Invalid Query Object. Not initialized or could not be created"),
             GPUQueryError::QueryResultError => write!(f, "Could not get query result")
            
        }
    }
}

impl std::error::Error for GPUQueryError{}