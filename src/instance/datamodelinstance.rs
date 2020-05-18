use serde_json::Value;

use super::containerinstance::ContainerInstance;
use super::leafinstance::LeafInstance;
use crate::model::datamodel::DataModel;

pub struct DataModelInstance<'a> {
    pub root: ContainerInstance<'a>,
}

impl<'a> DataModelInstance<'a> {
    pub fn new(model: &'a DataModel, value: &Value) -> DataModelInstance<'a> {
        let root_name = &model.root.name;
        let root = ContainerInstance::new(&model.root, &value[root_name], "".to_string(), None);

        DataModelInstance { root }
    }

    pub fn visit(&self, f: &dyn Fn(&LeafInstance) -> ()) {
        self.root.visit(f);
    }
}
