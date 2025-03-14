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

pub trait RuleValidator {
    fn name(&self) -> &str;
    fn run(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<Token>>,
        extra: Option<&Box<dyn Any + 'static>>,
    );
}

impl Debug for dyn RuleValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<dyn RuleValidator>())
    }
}
