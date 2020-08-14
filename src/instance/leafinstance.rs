use serde_json::Value;
use std::sync::Arc;

use super::util::*;
use crate::model::leaf::Leaf;

pub struct LeafInstance {
    pub parent: Parent,
    pub model: Arc<Leaf>,
    pub value: String,
}

impl LeafInstance {
    pub fn new(model: Arc<Leaf>, value: Value, parent: Parent) -> LeafInstance {
        let value_str = match value {
            Value::String(x) => x,
            Value::Number(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            _ => panic!("Leaf must have a string value!"),
        };

        LeafInstance {
            model,
            value: value_str,
            parent,
        }
    }

    pub fn get_path(&self) -> String {
        let parent_path = match &self.parent {
            Parent::ContainerData(x) => x.upgrade().unwrap().borrow().get_path(),
            Parent::ListChildData(x) => x.upgrade().unwrap().borrow().get_path(),
        };

        format!("{}/{}", parent_path, self.model.name)
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit)) {
        f(NodeToVisit::LeafInstance(self));
    }
}

impl Generated for LeafInstance {
    fn get_parent(&self) -> &Parent {
        &self.parent
    }
}
