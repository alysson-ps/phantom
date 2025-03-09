use crate::{config::RuleParams, err::rich::RichError, Token};
use chumsky::{input::Emitter, span::SimpleSpan};

use super::{Content, RuleValidator, Statement};

#[derive(Debug)]
pub struct EnforceNamespace;

impl RuleValidator for EnforceNamespace {
    fn run(
        &self,
        contents: &mut Content,
        params: RuleParams,
        emitter: &mut Emitter<RichError<Token>>,
    ) {
        let RuleParams(level, args) = &params;

        if level != "off" {
            let namespace = contents
                .statements
                .as_ref()
                .iter()
                .filter(|stmt| matches!(stmt, Statement::Namespace { .. }))
                .collect::<Vec<_>>();

            if namespace.clone().is_empty() {
                emitter.emit(RichError::custom(
                    SimpleSpan::new(0, 0),
                    "error".to_string(),
                    "No namespaces found",
                    false,
                ));
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
                        emitter.emit(RichError::custom(
                            *span,
                            "error".to_string(),
                            format!("Namespaces with brackets are not allowed"),
                            false,
                        ));
                    }
                }
            }
        }
    }
}
