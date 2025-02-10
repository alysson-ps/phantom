use std::collections::HashMap;

use crate::validates::{
    class_member_order::ClassMemberOrder, disallow_debug_functions::DisallowDebugFunctions,
    enforce_namespace::EnforceNamespace, line_length::LineLength,
    single_class_per_file::SingleClassPerFile, RuleValidator,
};

pub struct RuleFactory {
    rules: HashMap<String, Box<dyn RuleValidator>>,
}

impl RuleFactory {
    pub fn new() -> Self {
        let mut rules: HashMap<String, Box<dyn RuleValidator>> = HashMap::new();

        rules.insert("enforce-namespace".to_string(), Box::new(EnforceNamespace));
        rules.insert(
            "disallow-debug-functions".to_string(),
            Box::new(DisallowDebugFunctions),
        );
        rules.insert("line-length".to_string(), Box::new(LineLength));
        rules.insert(
            "single-class-per-file".to_string(),
            Box::new(SingleClassPerFile),
        );
        rules.insert("class-member-order".to_string(), Box::new(ClassMemberOrder));

        Self { rules }
    }

    pub fn get_rule(&self, name: &str) -> Option<&Box<dyn RuleValidator>> {
        self.rules.get(name)
    }
}
