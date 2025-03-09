use chumsky::input::Emitter;

use crate::{config::RuleParams, err::rich::RichError, Expr, Statement, Token};

use super::{Content, RuleValidator};

pub struct DisallowDebugFunctions;

impl RuleValidator for DisallowDebugFunctions {
    fn run<'a>(
        &self,
        contents: &'a mut Content<'a>,
        params: RuleParams,
        emitter: &mut Emitter<RichError<Token>>,
    ) {
        let RuleParams(level, args) = &params;

        if level != "off" {
            let statements = contents.statements.as_ref();

            statements.iter().for_each(|stmt| match stmt {
                Statement::Namespace { body, .. } => {
                    let token_ref = &mut contents.tokens.clone();

                    let mut new_contents = Box::new(Content {
                        statements: Box::new(body.clone()),
                        source: contents.source,
                        tokens: token_ref,
                    });

                    self.run(&mut new_contents, params.clone(), emitter);
                }
                Statement::Class { body, .. } => {
                    let token_ref = &mut contents.tokens.clone();

                    let mut new_contents = Box::new(Content {
                        statements: Box::new(body.clone()),
                        source: contents.source,
                        tokens: token_ref,
                    });

                    self.run(&mut new_contents, params.clone(), emitter);
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
                                        emitter.emit(RichError::custom(
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
