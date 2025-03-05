pub struct ClassMemberOrder;

use std::{collections::HashMap, ops::Index};

use chumsky::{error::Rich, input::Emitter};

use crate::{config::RuleParams, Statement, Token};

use super::{Content, RuleValidator};

impl RuleValidator for ClassMemberOrder {
    fn run(&self, contents: &Content, params: RuleParams, emitter: &mut Emitter<Rich<Token>>) {
        let RuleParams(level, args) = &params;

        if level != "off" {
            let statements = contents.statements.as_ref().unwrap();

            statements.as_ref().iter().for_each(|stmt| match stmt {
                Statement::Namespace { body, .. } => {
                    let new_contents = &Content {
                        statements: Some(Box::new(body.clone())),
                        source: contents.source,
                        tokens: contents.tokens.clone(),
                    };

                    self.run(new_contents, params.clone(), emitter)
                }
                Statement::Class { body, .. } => {
                    let methods = body
                        .iter()
                        .map(|stmt: &Statement<'_>| match stmt {
                            Statement::Method { name, span, .. } => ("methods", name, span),
                            Statement::Property { name, span, .. } => ("properties", name, span),
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
                                    emitter.emit(Rich::custom(
                                        **span,
                                        format!(
                                            "Member \"{}\" should be declared after {}",
                                            name, order.to_vec().index(last_index).to_string()
                                        ),
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
