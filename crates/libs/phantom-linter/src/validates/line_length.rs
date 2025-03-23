use itertools::Itertools;
use phantom_config::RuleParams;
use phantom_core::{rich::RichError, token::Token, Rule, Span};

#[derive(Debug)]
pub struct LineLength;

impl LineLength {
    pub fn run<'a, T>(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<'_, Token<'_>>>,
        extra: Option<T>,
    ) where
        T: AsRef<str>,
    {
        let RuleParams(level, args) = params;

        if level != "off" {
            if let Some(value) = args {
                let max = value.get("max").unwrap().as_u64().unwrap();

                if let Some(source) = extra {
                    let line_map: Vec<_> = source
                        .as_ref()
                        .lines()
                        .enumerate()
                        .map(|(i, line)| (i + 1, line.chars().count() + 1))
                        .sorted_by_key(|(i, _)| *i)
                        .collect();

                    let mut span_start = 0;

                    for (line_number, length) in &line_map {
                        if *length > (max as usize) {
                            errors.push(RichError::custom(
                                Span::new(span_start, span_start + length),
                                level.clone(),
                                format!(
                                    "Line {} has {} characters (max: {})",
                                    line_number, length, max
                                ),
                                Some(Rule::LineLength),
                            ));
                        }

                        span_start += length;
                    }
                }
            }
        }
    }
}
