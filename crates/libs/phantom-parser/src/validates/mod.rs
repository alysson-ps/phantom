pub mod class_member_order;
pub mod disallow_debug_functions;
pub mod enforce_namespace;
pub mod line_length;
pub mod single_class_per_file;

use std::any::Any;
use std::fmt::{write, Debug};
use std::sync::{Arc, Mutex};

use crate::config::RuleParams;
use crate::err::rich::RichError;
use crate::Token;

#[derive(Debug)]
pub struct Content {
    target: Arc<Mutex<dyn Any + Send + Sync>>,
}

impl Content {
    pub fn new<T: Any + Send + Sync>(value: T) -> Self {
        Self {
            target: Arc::new(Mutex::new(value)),
        }
    }

    pub fn get<T: Any>(&self) -> Option<T>
    where
        T: Clone, // Precisamos clonar o valor para retornar uma nova instância
    {
        let lock = self.target.lock().unwrap(); // Mantém o lock vivo dentro do escopo
        lock.downcast_ref::<T>().cloned() // Clonamos para evitar referência inválida
    }
}

pub trait RuleValidator {
    fn run<'a>(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<Token>>,
        extra: Option<Content>,
    );
}

impl Debug for dyn RuleValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<dyn RuleValidator>())
    }
}
