#[derive(Debug)]
pub struct ClassMemberOrder;

use std::{collections::HashMap, ops::Index};

use phantom_config::RuleParams;
use phantom_core::{rich::RichError, token::Token, Rule, Span, Statement};

impl ClassMemberOrder {
    pub fn run<'a, T>(
        &self,
        params: RuleParams,
        errors: &mut Vec<RichError<Token>>,
        extra: Option<T>,
    ) where
        T: AsRef<[Statement<'a>]>,
    {
        let RuleParams(level, args) = &params;

        if level != "off" {
            if let Some(statements) = extra {
                statements
                    .as_ref()
                    .iter()
                    // .filter(|stmt| matches!(stmt, Statement::Class { .. }))
                    .for_each(|stmt| match stmt {
                        Statement::Namespace { body, .. } => {
                            let extra = Some(body);

                            self.run(params.clone(), errors, extra);
                        }
                        Statement::Class { body, .. } => {
                            let methods = body
                                .iter()
                                .map(|stmt: &Statement<'_>| match stmt {
                                    Statement::Method { name, span, .. } => ("methods", name, span),
                                    Statement::Property { name, span, .. } => {
                                        ("properties", name, span)
                                    }
                                    _ => todo!(),
                                })
                                .collect::<Vec<_>>();

                            if let Some(value) = args {
                                let order = value.get("order").unwrap().as_array().unwrap();

                                let mut sort = HashMap::new();
                                for (i, member_type) in order.iter().enumerate() {
                                    // dbg!(i, member_type.as_str().unwrap());
                                    sort.insert(member_type.as_str().unwrap(), i);
                                }

                                let mut index = 0; // Começa com o menor índice possível

                                methods.iter().for_each(|(member_type, name, span)| {
                                    if let Some(&current_index) = sort.get(member_type) {
                                        if current_index < index {
                                            if let Some(search) =
                                                order.to_vec().index(index).as_str()
                                            {
                                                let order_span = methods
                                                    .iter()
                                                    .filter(|(type_, _, _)| *type_ == search)
                                                    .map(|(_, _, span)| span)
                                                    .collect::<Vec<_>>()
                                                    .first()
                                                    .cloned();

                                                errors.push(RichError::custom(
                                                    **span,
                                                    "error".to_string(),
                                                    format!(
                                                        "Member \"{}\" should be declared after {}",
                                                        name, search
                                                    ),
                                                    Some(Rule::ClassMemberOrder(
                                                        order
                                                            .iter()
                                                            .map(|v| {
                                                                v.as_str().unwrap().to_string()
                                                            })
                                                            .collect(),
                                                        **order_span.unwrap(),
                                                    )),
                                                ));
                                            }
                                        }
                                        index = current_index;
                                    }
                                });
                            }
                        }
                        _ => {}
                    });

                // if let Some(value) = args {}
            }
        }
    }
}
