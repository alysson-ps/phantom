use crate::{
    config::RuleParams,
    err::rich::RichError,
    validates::{
        // class_member_order::ClassMemberOrder, disallow_debug_functions::DisallowDebugFunctions,
        // enforce_namespace::EnforceNamespace, line_length::LineLength,
        single_class_per_file::SingleClassPerFile, RuleValidator,
    },
    Statement, Token,
};

// pub struct RuleFactory {
//     rules: HashMap<String, Box<dyn RuleValidator>>,
// }

// impl RuleFactory {
//     pub fn new() -> Self {
//         let mut rules: HashMap<String, Box<dyn RuleValidator>> = HashMap::new();

//         rules.insert("enforce-namespace".to_string(), Box::new(EnforceNamespace));
//         rules.insert(
//             "disallow-debug-functions".to_string(),
//             Box::new(DisallowDebugFunctions),
//         );
//         rules.insert("line-length".to_string(), Box::new(LineLength));
//         rules.insert(
//             "single-class-per-file".to_string(),
//             Box::new(SingleClassPerFile),
//         );
//         rules.insert("class-member-order".to_string(), Box::new(ClassMemberOrder));

//         Self { rules }
//     }

//     pub fn get_rule(&self, name: &str) -> Option<&Box<dyn RuleValidator>> {
//         self.rules.get(name)
//     }
// }
#[derive(Debug)]
pub enum RuleFactory {
    // EnforceNamespace(EnforceNamespace),
    // DisallowDebugFunctions(DisallowDebugFunctions),
    // LineLength(LineLength),
    SingleClassPerFile(SingleClassPerFile),
    // ClassMemberOrder(ClassMemberOrder),
}

impl RuleFactory {
    pub fn get(name: &str) -> Self {
        match name {
            // "enforce-namespace" => RuleFactory::EnforceNamespace(EnforceNamespace),
            // "disallow-debug-functions" => {
            //     RuleFactory::DisallowDebugFunctions(DisallowDebugFunctions)
            // }
            // "line-length" => RuleFactory::LineLength(LineLength),
            "single-class-per-file" => RuleFactory::SingleClassPerFile(SingleClassPerFile),
            // "class-member-order" => RuleFactory::ClassMemberOrder(ClassMemberOrder),
            _ => panic!("Invalid rule name"),
        }
    }

    pub fn run(&self, params: RuleParams, errors: &mut Vec<RichError<Token>>) {
        match self {
            // RuleFactory::EnforceNamespace(rule) => rule.run::<Vec<Statement>>(params, errors, None),
            // RuleFactory::DisallowDebugFunctions(rule) => {
            //     rule.run::<Vec<Statement>>(params, errors, None)
            // }
            // RuleFactory::LineLength(rule) => rule.run::<&str>(params, errors, None),
            RuleFactory::SingleClassPerFile(rule) => {
                rule.run::<Vec<Statement>>(params, errors, None)
            }
            // RuleFactory::ClassMemberOrder(rule) => rule.run::<Vec<Statement>>(params, errors, None),
        }
    }
}
