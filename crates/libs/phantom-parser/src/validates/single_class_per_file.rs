pub struct SingleClassPerFile;

use chumsky::input::Emitter;

use crate::{config::RuleParams, err::rich::RichError, Token};

use super::{Content, RuleValidator, Statement};

impl RuleValidator for SingleClassPerFile {
    fn run(&self, contents: &mut Content, params: RuleParams, emitter: &mut Emitter<RichError<Token>>) {
        let RuleParams(level, _) = &params;

        if level != "off" {
            let statements = contents.statements.as_ref();

            statements.iter().for_each(|stmt| match stmt {
                Statement::Namespace { body, span, .. } => {
                    let class_statement = body
                        .iter()
                        .filter(|stmt| matches!(stmt, Statement::Class { .. }))
                        .collect::<Vec<_>>();

                    if class_statement.len() > 1 {
                        emitter.emit(RichError::custom(
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
                        emitter.emit(RichError::custom(
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
