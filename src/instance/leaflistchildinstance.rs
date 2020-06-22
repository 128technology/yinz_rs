use serde_json::Value;
use std::sync::Arc;

use crate::model::leaflist::LeafList;

pub struct LeafListChildInstance {
    pub model: Arc<LeafList>,
    pub value: String,
}

impl<'a> LeafListChildInstance {
    pub fn new(model: Arc<LeafList>, value: &Value) -> LeafListChildInstance {
        let value_str = match value {
            Value::String(x) => x.clone(),
            Value::Number(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            _ => panic!("Leaf list value must have a string value!"),
        };

        LeafListChildInstance {
            model,
            value: value_str,
        }
    }
}
