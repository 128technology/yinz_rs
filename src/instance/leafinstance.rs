use serde_json::Value;
use std::sync::Arc;

use super::util::*;
use crate::model::leaf::Leaf;

pub struct LeafInstance {
    pub parent: Parent,
    pub model: Arc<Leaf>,
    pub value: String,
    pub path: String,
}

impl LeafInstance {
    pub fn new(
        model: Arc<Leaf>,
        value: &Value,
        parent_path: String,
        parent: Parent,
    ) -> LeafInstance {
        let name = &model.name;
        let value_str = match value {
            Value::String(x) => x.clone(),
            Value::Number(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            _ => panic!("Leaf must have a string value!"),
        };

        LeafInstance {
            model: model.clone(),
            value: value_str,
            path: format!("{}/{}", parent_path, name),
            parent,
        }
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit) -> ()) {
        f(NodeToVisit::LeafInstance(self));
    }
}

impl Generated for LeafInstance {
    fn get_parent(&self) -> &Parent {
        &self.parent
    }
}
