use std::any::Any;

use crate::{config::RuleParams, err::rich::RichError, Statement, Token};
use chumsky::span::SimpleSpan;

use super::RuleValidator;

#[derive(Debug)]
pub struct EnforceNamespace;

impl RuleValidator for EnforceNamespace {
    fn name(&self) -> &str {
        "enforce_namespace"
    }

    fn run(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<Token>>,
        extra: Option<&Box<dyn Any>>,
    ) {
        let RuleParams(level, args) = &params;

        if level != "off" {
            if let Some(statements) = extra.unwrap().downcast_ref::<Vec<Statement>>() {
                let namespace = statements
                    .iter()
                    .filter(|stmt| matches!(stmt, Statement::Namespace { .. }))
                    .collect::<Vec<_>>();

                if namespace.clone().is_empty() {
                    errors.push(RichError::custom(
                        SimpleSpan::new(0, 0),
                        "error".to_string(),
                        "No namespaces found",
                        false,
                    ));
                }

                if let Some(value) = args {
                    let allow_brackets =
                        value.get("allow-brackets").unwrap().as_bool().unwrap_or(false);

                    if let Some(Statement::Namespace {
                        with_brackets,
                        span,
                        ..
                    }) = namespace.first()
                    {
                        if !allow_brackets && *with_brackets {
                            errors.push(RichError::custom(
                                *span,
                                "error".to_string(),
                                format!("Namespaces with brackets are not allowed"),
                                false,
                            ));
                        }
                    }
                }
            }
        }
    }
}
