use chumsky::span::SimpleSpan;
use itertools::Itertools;

use crate::{config::RuleParams, err::rich::RichError, Token};

use super::{Content, RuleValidator};

#[derive(Debug)]
pub struct LineLength;

impl RuleValidator for LineLength {
    fn run(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<'_, Token<'_>>>,
        extra: Option<Content>,
    ) {
        let RuleParams(level, args) = params;

        if level != "off" {
            if let Some(value) = args {
                let max = value.get("max").unwrap().as_u64().unwrap();

                if let Some(source) = extra.unwrap().get::<&str>() {
                    let line_map: Vec<_> = source
                        .lines()
                        .enumerate()
                        .map(|(i, line)| (i + 1, line.chars().count() + 1))
                        .sorted_by_key(|(i, _)| *i)
                        .collect();

                    let mut span_start = 0;

                    for (line_number, length) in &line_map {
                        if *length > (max as usize) {
                            errors.push(RichError::custom(
                                SimpleSpan::new(span_start, span_start + length),
                                level.clone(),
                                format!(
                                    "Line {} has {} characters (max: {})",
                                    line_number, length, max
                                ),
                                true,
                            ));
                        }

                        span_start += length;
                    }
                }
            }
        }
    }
}
