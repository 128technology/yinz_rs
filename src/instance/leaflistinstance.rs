use serde_json::Value;
use std::sync::Arc;

use super::leaflistchildinstance::LeafListChildInstance;
use super::util::*;
use crate::model::leaflist::LeafList;

pub struct LeafListInstance {
    pub parent: Parent,
    pub model: Arc<LeafList>,
    pub children: Vec<LeafListChildInstance>,
    pub path: String,
}

impl LeafListInstance {
    pub fn new(
        model: Arc<LeafList>,
        value: &Value,
        parent_path: String,
        parent: Parent,
    ) -> LeafListInstance {
        let name = &model.name;
        let value_arr = match value {
            Value::Array(x) => x,
            _ => panic!("Leaf list must have an array value!"),
        };

        let mut children: Vec<LeafListChildInstance> = Vec::new();

        for leaf_list_value in value_arr {
            children.push(LeafListChildInstance::new(model.clone(), leaf_list_value));
        }

        LeafListInstance {
            model: model.clone(),
            children,
            path: format!("{}/{}", parent_path, name),
            parent,
        }
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit) -> ()) {
        f(NodeToVisit::LeafListInstance(self));
    }
}

impl Generated for LeafListInstance {
    fn get_parent(&self) -> &Parent {
        &self.parent
    }
}
