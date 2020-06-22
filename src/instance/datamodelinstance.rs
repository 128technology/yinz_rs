use serde_json::Value;
use std::sync::Arc;

use super::containerinstance::ContainerInstance;
use super::util::*;
use crate::model::datamodel::DataModel;

pub struct DataModelInstance {
    pub root: ContainerInstance,
}

impl DataModelInstance {
    pub fn new(model: Arc<DataModel>, value: &Value) -> DataModelInstance {
        let root_name = &model.root.name;
        let root =
            ContainerInstance::new(model.root.clone(), &value[root_name], "".to_string(), None);

        DataModelInstance { root }
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit) -> ()) {
        self.root.visit(f);
    }
}
