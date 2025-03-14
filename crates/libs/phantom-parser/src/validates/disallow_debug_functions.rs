use std::any::Any;

use crate::{config::RuleParams, err::rich::RichError, Expr, Statement, Token};

use super::RuleValidator;

#[derive(Debug)]
pub struct DisallowDebugFunctions;

impl RuleValidator for DisallowDebugFunctions {
    fn name(&self) -> &str {
        "disallow_debug_functions"
    }

    fn run<'a>(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<Token>>,
        extra: Option<&Box<dyn Any>>,
    ) {
        let RuleParams(level, args) = &params;

        if level != "off" {
            if let Some(statements) = extra.unwrap().downcast_ref::<Vec<Statement>>() {
                let statements_clone = statements.clone();

                statements_clone.into_iter().for_each(|stmt| match stmt {
                    Statement::Namespace { body, .. } => {
                        let extra = Some(Box::new(body) as Box<dyn Any>);
                        self.run(params.clone(), errors, extra.as_ref());
                    }
                    Statement::Class { body, .. } => {
                        let extra = Some(Box::new(body) as Box<dyn Any>);
                        self.run(params.clone(), errors, extra.as_ref());
                    }
                    Statement::Method { body, .. } => {
                        body.iter().for_each(|expr| match &expr {
                            Expr::Call { func, span, .. } => {
                                if let Expr::Local(name) = &**func {
                                    if let Some(value) = args {
                                        if value
                                            .get("functions")
                                            .unwrap()
                                            .as_array()
                                            .unwrap()
                                            .contains(&&name.to_string().into())
                                        {
                                            errors.push(RichError::custom(
                                                *span,
                                                "error".to_string(),
                                                format!(
                                                    "The use of debug function '{}' is not allowed",
                                                    name
                                                ),
                                                true,
                                            ));
                                        }
                                    }
                                }
                            }
                            _ => {}
                        });
                    }
                    _ => {}
                });
            }
        }
    }
}
