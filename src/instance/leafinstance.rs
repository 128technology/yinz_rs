use serde_json::Value;

use super::util::*;
use crate::model::leaf::Leaf;

pub struct LeafInstance<'a> {
    pub parent: Parent<'a>,
    pub model: &'a Leaf,
    pub value: String,
    pub path: String,
}

impl<'a> LeafInstance<'a> {
    pub fn new(
        model: &'a Leaf,
        value: &Value,
        parent_path: String,
        parent: Parent<'a>,
    ) -> LeafInstance<'a> {
        let value_str = match value {
            Value::String(x) => x.clone(),
            Value::Number(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            _ => panic!("Leaf must have a string value!"),
        };

        LeafInstance {
            model,
            value: value_str,
            path: format!("{}/{}", parent_path, model.name),
            parent,
        }
    }

    pub fn visit(&self, f: &dyn Fn(&LeafInstance) -> ()) {
        f(self);
    }

    pub fn is_generated(&self) -> bool {
        match &self.parent {
            Parent::ListChildData(p) => p.upgrade().unwrap().read().unwrap().is_generated(),
            _ => false,
        }
    }
}
