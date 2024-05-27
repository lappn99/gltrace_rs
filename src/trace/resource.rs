use types::*;

use crate::types::types;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum TransactionType {
    CreateShader,
    UpdateShaderSource(String)
}

#[derive(Debug, Clone)]
pub struct ResourceTransaction{

    pub resource: GLuint,
    pub transaction_type: TransactionType,
    pub entry: Rc<super::TraceEntry>,
}

