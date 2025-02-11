use chumsky::{error::Rich, input::Emitter, span::SimpleSpan};

use crate::{config::RuleParams, err::LintError, Expr, Statement, Token};

use super::RuleValidator;

pub struct DisallowDebugFunctions;

impl RuleValidator for DisallowDebugFunctions {
    fn run(
        &self,
        _tokens: &Vec<(Token, SimpleSpan)>,
        statements: &Vec<Statement>,
        params: RuleParams,
        emitter: &mut Emitter<Rich<Token>>,
    ) {
        let RuleParams(level, args) = &params;

        if level != "off" {
            statements.iter().for_each(|stmt| match stmt {
                Statement::Namespace { body, .. } => {
                    self.run(_tokens, body, params.clone(), emitter);
                }
                Statement::Class { body, .. } => {
                    self.run(_tokens, body, params.clone(), emitter);
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
                                        emitter.emit(Rich::custom(
                                            *span,
                                            format!(
                                                "The use of debug function '{}' is not allowed",
                                                name
                                            ),
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
