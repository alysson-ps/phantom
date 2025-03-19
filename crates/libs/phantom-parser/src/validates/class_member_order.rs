#[derive(Debug)]
pub struct ClassMemberOrder;

use std::{collections::HashMap, ops::Index};

use crate::{config::RuleParams, err::rich::RichError, Statement, Token};

use super::RuleValidator;

impl RuleValidator for ClassMemberOrder {
    fn name(&self) -> &str {
        "class_member_order"
    }

    fn run<'a, T>(&self, params: RuleParams, errors: &mut Vec<RichError<Token>>, extra: Option<T>)
    where
        T: AsRef<[Statement<'a>]>,
    {
        let RuleParams(level, args) = &params;

        if level != "off" {
            if let Some(statements) = extra {
                let statements_cloned = statements.as_ref().clone();

                statements_cloned.as_ref().iter().for_each(|stmt| match stmt {
                    Statement::Namespace { body, .. } => {
                        self.run(params.clone(), errors, Some(body))
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

                            let mut last_index = 0; // Começa com o menor índice possível

                            methods.iter().for_each(|(member_type, name, span)| {
                                if let Some(&current_index) = sort.get(member_type) {
                                    if current_index < last_index {
                                        errors.push(RichError::custom(
                                            **span,
                                            "error".to_string(),
                                            format!(
                                                "Member \"{}\" should be declared after {}",
                                                name,
                                                order.to_vec().index(last_index).to_string()
                                            ),
                                            true,
                                        ));
                                    }
                                    last_index = current_index;
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
