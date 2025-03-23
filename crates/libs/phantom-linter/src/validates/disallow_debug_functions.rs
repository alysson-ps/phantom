use std::fmt::Debug;

use phantom_config::RuleParams;
use phantom_core::{rich::RichError, token::Token, Expr, Rule, Statement};

#[derive(Debug)]
pub struct DisallowDebugFunctions;

impl DisallowDebugFunctions {
    pub fn run<'a, T>(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<Token>>,
        extra: Option<T>,
    ) where
        T: AsRef<[Statement<'a>]>,
    {
        let RuleParams(level, args) = &params;

        if level != "off" {
            if let Some(statements) = extra {
                statements.as_ref().iter().for_each(|stmt| match stmt {
                    Statement::Namespace { body, .. } => {
                        let extra = Some(body);

                        self.run(params.clone(), errors, extra);
                    }
                    Statement::Class { body, .. } => {
                        let extra = Some(body);

                        self.run(params.clone(), errors, extra);
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
                                                Some(Rule::DisallowDebugFunctions),
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
