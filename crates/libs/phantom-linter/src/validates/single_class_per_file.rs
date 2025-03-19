use phantom_config::RuleParams;
use phantom_core::{rich::RichError, Statement, Token};

#[derive(Debug)]
pub struct SingleClassPerFile;

impl SingleClassPerFile {
    pub fn run<'a, T>(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<Token>>,
        extra: Option<T>,
    ) where
        T: AsRef<[Statement<'a>]>,
    {
        let RuleParams(level, _) = &params;
        dbg!(level);

        if level != "off" {
            if let Some(statements) = extra {
                statements.as_ref().iter().for_each(|stmt| match stmt {
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
                            .as_ref()
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
