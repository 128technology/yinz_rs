use serde_json::Value;
use std::sync::Arc;

use super::leaflistchildinstance::LeafListChildInstance;
use super::util::*;
use crate::model::leaflist::LeafList;

pub struct LeafListInstance {
    pub parent: Parent,
    pub model: Arc<LeafList>,
    pub children: Vec<LeafListChildInstance>,
}

impl LeafListInstance {
    pub fn new(model: Arc<LeafList>, value: Value, parent: Parent) -> LeafListInstance {
        let value_arr = match value {
            Value::Array(x) => x,
            _ => panic!("Leaf list must have an array value!"),
        };

        let mut children: Vec<LeafListChildInstance> = Vec::new();

        for leaf_list_value in value_arr {
            children.push(LeafListChildInstance::new(model.clone(), leaf_list_value));
        }

        LeafListInstance {
            model,
            children,
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
        f(NodeToVisit::LeafListInstance(self));
    }
}

impl Generated for LeafListInstance {
    fn get_parent(&self) -> &Parent {
        &self.parent
    }
}
