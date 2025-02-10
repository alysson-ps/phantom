use std::collections::HashMap;

use crate::validates::{
    disallow_debug_functions::DisallowDebugFunctions, enforce_namespace::EnforceNamespace,
    RuleValidator,
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

        Self { rules }
    }

    pub fn get_rule(&self, name: &str) -> Option<&Box<dyn RuleValidator>> {
        self.rules.get(name)
    }
}
