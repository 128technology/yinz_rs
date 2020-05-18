use serde_json::Value;

use super::leafinstance::LeafInstance;
use super::leaflistchildinstance::LeafListChildInstance;
use super::util::*;
use crate::model::leaflist::LeafList;

pub struct LeafListInstance<'a> {
    pub parent: Parent<'a>,
    pub model: &'a LeafList,
    pub children: Vec<LeafListChildInstance<'a>>,
    pub path: String,
}

impl<'a> LeafListInstance<'a> {
    pub fn new(
        model: &'a LeafList,
        value: &Value,
        parent_path: String,
        parent: Parent<'a>,
    ) -> LeafListInstance<'a> {
        let value_arr = match value {
            Value::Array(x) => x,
            _ => panic!("Leaf list must have an array value!"),
        };

        let mut children: Vec<LeafListChildInstance> = Vec::new();

        for leaf_list_value in value_arr {
            children.push(LeafListChildInstance::new(model, leaf_list_value));
        }

        LeafListInstance {
            model,
            children,
            path: format!("{}/{}", parent_path, model.name),
            parent,
        }
    }

    pub fn visit(&self, _f: &dyn Fn(&LeafInstance) -> ()) {}
}
