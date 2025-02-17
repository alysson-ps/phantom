use chumsky::{error::Rich, input::Emitter};

use crate::{config::RuleParams, Expr, Statement, Token};

use super::{Content, RuleValidator};

pub struct DisallowDebugFunctions;

impl RuleValidator for DisallowDebugFunctions {
    fn run(&self, contents: &Content, params: RuleParams, emitter: &mut Emitter<Rich<Token>>) {
        let RuleParams(level, args) = &params;

        if level != "off" {
            let statements = contents.statements.as_ref().unwrap();

            statements.as_ref().iter().for_each(|stmt| match stmt {
                Statement::Namespace { body, .. } => {
                    let new_contents = &Content {
                        statements: Some(Box::new(body)),
                        source: contents.source,
                        tokens: contents.tokens,
                    };

                    self.run(new_contents, params.clone(), emitter);
                }
                Statement::Class { body, .. } => {
                    let new_contents = &Content {
                        statements: Some(Box::new(body)),
                        source: contents.source,
                        tokens: contents.tokens,
                    };

                    self.run(new_contents, params.clone(), emitter);
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
