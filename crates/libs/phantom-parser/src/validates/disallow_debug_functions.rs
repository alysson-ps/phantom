use crate::{config::RuleParams, err::rich::RichError, Expr, Statement, Token};

use super::{Content, RuleValidator};

#[derive(Debug)]
pub struct DisallowDebugFunctions;

impl RuleValidator for DisallowDebugFunctions {
    fn run<'a>(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<Token>>,
        extra: Option<Content>,
    ) {
        let RuleParams(level, args) = &params;

        if level != "off" {
            if let Some(statements) = extra.unwrap().get::<Vec<Statement>>() {
                let statements_clone = statements.clone();

                statements_clone.into_iter().for_each(|stmt| match stmt {
                    Statement::Namespace { body, .. } => {
                        self.run(params.clone(), errors, Some(Content::new(body)));
                    }
                    Statement::Class { body, .. } => {
                        self.run(params.clone(), errors, Some(Content::new(body)));
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
