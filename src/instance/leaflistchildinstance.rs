use serde_json::Value;

use crate::model::leaflist::LeafList;

pub struct LeafListChildInstance<'a> {
    pub model: &'a LeafList,
    pub value: String,
}

impl<'a> LeafListChildInstance<'a> {
    pub fn new(model: &'a LeafList, value: &Value) -> LeafListChildInstance<'a> {
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
