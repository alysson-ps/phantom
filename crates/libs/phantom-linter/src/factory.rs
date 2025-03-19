use phantom_config::RuleParams;
use phantom_core::{rich::RichError, Program, Statement, Token};

use crate::validates::{
    class_member_order::ClassMemberOrder, disallow_debug_functions::DisallowDebugFunctions,
    enforce_namespace::EnforceNamespace, line_length::LineLength,
    single_class_per_file::SingleClassPerFile,
};

#[derive(Debug)]
pub enum RuleFactory {
    EnforceNamespace(EnforceNamespace),
    DisallowDebugFunctions(DisallowDebugFunctions),
    LineLength(LineLength),
    SingleClassPerFile(SingleClassPerFile),
    ClassMemberOrder(ClassMemberOrder),
}

#[derive(Clone)]
pub struct Extra<'a> {
    pub source: &'a str,
    pub program: Program<'a>,
    pub tokens: Vec<Token<'a>>,
}

impl RuleFactory {
    pub fn get(name: &str) -> Self {
        match name {
            "enforce-namespace" => RuleFactory::EnforceNamespace(EnforceNamespace),
            "disallow-debug-functions" => {
                RuleFactory::DisallowDebugFunctions(DisallowDebugFunctions)
            }
            "line-length" => RuleFactory::LineLength(LineLength),
            "single-class-per-file" => RuleFactory::SingleClassPerFile(SingleClassPerFile),
            "class-member-order" => RuleFactory::ClassMemberOrder(ClassMemberOrder),
            _ => panic!("Invalid rule name"),
        }
    }

    pub fn run<'a>(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<Token>>,
        extra: Box<Extra<'a>>,
    ) {
        match self {
            RuleFactory::EnforceNamespace(rule) => {
                rule.run::<Vec<Statement>>(params, errors, Some(extra.program.statements))
            }
            RuleFactory::DisallowDebugFunctions(rule) => {
                rule.run::<Vec<Statement>>(params, errors, Some(extra.program.statements))
            }
            RuleFactory::LineLength(rule) => rule.run::<&str>(params, errors, Some(&extra.source)),
            RuleFactory::SingleClassPerFile(rule) => {
                rule.run::<Vec<Statement>>(params, errors, Some(extra.program.statements))
            }
            RuleFactory::ClassMemberOrder(rule) => {
                rule.run::<Vec<Statement>>(params, errors, Some(extra.program.statements))
            }
        }
    }
}
