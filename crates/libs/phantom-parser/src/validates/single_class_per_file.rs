#[derive(Debug)]
pub struct SingleClassPerFile;

use crate::{config::RuleParams, err::rich::RichError, Statement, Token};

use super::{Content, RuleValidator};

impl RuleValidator for SingleClassPerFile {
    fn run(&self, params: RuleParams, errors: &mut Vec<RichError<Token>>, extra: Option<Content>) {
        let RuleParams(level, _) = &params;

        if level != "off" {
            if let Some(statements) = extra.unwrap().get::<Vec<Statement>>() {
                statements.iter().for_each(|stmt| match stmt {
                    Statement::Namespace { body, span, .. } => {
                        let class_statement = body
                            .iter()
                            .filter(|stmt| matches!(stmt, Statement::Class { .. }))
                            .collect::<Vec<_>>();

                        if class_statement.len() > 1 {
                            errors.push(RichError::custom(
                                *span,
                                "error".to_string(),
                                "More than one class per namespace is not allowed",
                                false,
                            ));
                        }
                    }
                    Statement::Class { span, .. } => {
                        let class_statement = statements
                            .iter()
                            .filter(|stmt| matches!(stmt, Statement::Class { .. }))
                            .collect::<Vec<_>>();

                        if class_statement.len() > 1 {
                            errors.push(RichError::custom(
                                *span,
                                "error".to_string(),
                                "More than one class per file is not allowed",
                                false,
                            ));
                        }
                    }
                    _ => {}
                });
            }
        }
    }
}
