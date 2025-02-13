use crate::{config::RuleParams, Token};
use chumsky::{error::Rich, input::Emitter, span::SimpleSpan};

use super::{Content, RuleValidator, Statement};

#[derive(Debug)]
pub struct EnforceNamespace;

impl RuleValidator for EnforceNamespace {
    fn run(&self, contents: &Content, params: RuleParams, emitter: &mut Emitter<Rich<Token>>) {
        let RuleParams(level, args) = &params;

        if level != "off" {
            let namespace = contents
                .statements
                .as_ref()
                .unwrap()
                .iter()
                .filter(|stmt| matches!(stmt, Statement::Namespace { .. }))
                .collect::<Vec<_>>();

            if namespace.clone().is_empty() {
                emitter.emit(Rich::custom(SimpleSpan::new(0, 0), "No namespaces found"));
            }

            if let Some(value) = args {
                let allow_brackets =
                    value.get("allow-brackets").unwrap().as_bool().unwrap_or(false);

                if let Some(Statement::Namespace {
                    with_brackets,
                    span,
                    ..
                }) = namespace.first()
                {
                    if !allow_brackets && *with_brackets {
                        emitter.emit(Rich::custom(
                            *span,
                            format!("Namespaces with brackets are not allowed"),
                        ));
                    }
                }
            }
        }
    }
}
