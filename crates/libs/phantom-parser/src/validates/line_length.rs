use chumsky::{error::Rich, input::Emitter, span::SimpleSpan};
use itertools::Itertools;

use crate::{config::RuleParams, err::LintError, Token};

use super::{Content, RuleValidator};

pub struct LineLength;

impl RuleValidator for LineLength {
    fn run(
        &self,
        contents: &Content,
        params: RuleParams,
        emitter: &mut Emitter<Rich<'_, Token<'_>>>,
    ) {
        let RuleParams(level, args) = params;

        if level != "off" {
            if let Some(value) = args {
                let max = value.get("max").unwrap().as_u64().unwrap();

                let line_map: Vec<_> = contents
                    .source
                    .unwrap()
                    .lines()
                    .enumerate()
                    .map(|(i, line)| (i + 1, line.chars().count() + 1))
                    .sorted_by_key(|(i, _)| *i)
                    .collect();

                let mut span_start = 0;

                for (line_number, length) in &line_map {
                    if *length > (max as usize) {
                        let error: Rich<Token> = LintError {
                            level: level.clone(),
                            message: format!(
                                "Line {} has {} characters (max: {})",
                                line_number, length, max
                            ),
                            span: SimpleSpan::new(span_start, span_start + length),
                        }
                        .into();

                        emitter.emit(error);
                    }

                    span_start += length;
                }
            }
        }
    }
}
