use phantom_config::RuleParams;
use phantom_core::{rich::RichError, Expr, Statement, Token};

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

        dbg!(level);
        dbg!(args);

        if level != "off" {
            if let Some(statements) = extra {
                statements
                    .as_ref()
                    .iter()
                    .filter(|stmt| matches!(stmt, Statement::Method { .. }))
                    .for_each(|stmt| match stmt {
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
