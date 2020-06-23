use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::listchildinstance::ListChildInstance;
use super::util::*;
use crate::model::list::List;

pub struct ListData {
    pub parent: Parent,
    pub model: Arc<List>,
    pub children: Option<Arc<RwLock<HashMap<String, ListChildInstance>>>>,
    pub path: String,
}

type Link = Arc<RwLock<ListData>>;

pub struct ListInstance(Link);

impl Clone for ListInstance {
    fn clone(&self) -> Self {
        ListInstance(Arc::clone(&self.0))
    }
}

impl PartialEq for ListInstance {
    fn eq(&self, other: &ListInstance) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl ListInstance {
    pub fn new(
        model: Arc<List>,
        value: &Value,
        parent_path: String,
        parent: Parent,
    ) -> ListInstance {
        let value_arr = match value {
            Value::Array(x) => x,
            _ => panic!("List must have an array value!"),
        };

        let path = format!("{}/{}", parent_path, model.name);
        let child_path = path.clone();

        let instance = ListInstance(Arc::new(RwLock::new(ListData {
            model: model.clone(),
            children: None,
            path,
            parent,
        })));

        let mut children: HashMap<String, ListChildInstance> = HashMap::new();

        for list_value in value_arr.iter() {
            let children_parent = Arc::downgrade(&instance.0);
            let child_instance = ListChildInstance::new(
                model.clone(),
                list_value,
                child_path.clone(),
                children_parent,
            );
            children.insert(child_instance.get_key(), child_instance);
        }

        instance.0.write().unwrap().children = Some(Arc::new(RwLock::new(children)));

        instance
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit) -> ()) {
        for child in self
            .0
            .read()
            .unwrap()
            .children
            .as_ref()
            .unwrap()
            .read()
            .unwrap()
            .values()
        {
            child.visit(f);
        }
    }
}
