use types::*;

use crate::types::types;
use std::{collections::HashMap, hash::Hash, rc::Rc};

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


impl ResourceTransaction {
    pub fn group_transactions(transactions: &[Self]) -> HashMap<types::GLuint, Vec<&Self>> {

        let mut groups = HashMap::new();
        
        for transaction in transactions {
            groups.entry(transaction.resource).or_insert(Vec::new()).push(transaction)
        }

        groups

        

    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::CreateShader => {
                writeln!(f,"Created shader")
            },
            TransactionType::UpdateShaderSource(source) => {
                writeln!(f,"Updated shader source: {}", source)
            }
        }
    }
}