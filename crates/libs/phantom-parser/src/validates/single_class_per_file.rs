pub struct SingleClassPerFile;

use chumsky::{error::Rich, input::Emitter};

use crate::{config::RuleParams, Token};

use super::{Content, RuleValidator, Statement};

impl RuleValidator for SingleClassPerFile {
    fn run(&self, contents: &Content, params: RuleParams, emitter: &mut Emitter<Rich<Token>>) {
        let RuleParams(level, _) = &params;

        if level != "off" {
            let statements = contents.statements.as_ref().unwrap();

            statements.as_ref().iter().for_each(|stmt| match stmt {
                Statement::Namespace { body, span, .. } => {
                    let class_statement = body
                        .iter()
                        .filter(|stmt| matches!(stmt, Statement::Class { .. }))
                        .collect::<Vec<_>>();

                    if class_statement.len() > 1 {
                        emitter.emit(Rich::custom(
                            *span,
                            "More than one class per namespace is not allowed",
                        ));
                    }
                }
                Statement::Class { span, .. } => {
                    let class_statement = statements
                        .iter()
                        .filter(|stmt| matches!(stmt, Statement::Class { .. }))
                        .collect::<Vec<_>>();

                    if class_statement.len() > 1 {
                        emitter.emit(Rich::custom(
                            *span,
                            "More than one class per file is not allowed",
                        ));
                    }
                }
                _ => {}
            });
        }
    }
}
