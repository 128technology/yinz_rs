use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use super::listchildinstance::ListChildInstance;
use super::util::*;
use crate::model::list::List;

pub struct ListData {
    pub parent: Parent,
    pub model: Arc<List>,
    pub children: Option<Rc<RefCell<HashMap<String, ListChildInstance>>>>,
}

type Link = Rc<RefCell<ListData>>;

pub struct ListInstance(Link);

impl Clone for ListInstance {
    fn clone(&self) -> Self {
        ListInstance(Rc::clone(&self.0))
    }
}

impl PartialEq for ListInstance {
    fn eq(&self, other: &ListInstance) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl ListInstance {
    pub fn new(model: Arc<List>, value: Value, parent: Parent) -> ListInstance {
        let value_arr = match value {
            Value::Array(x) => x,
            _ => panic!("List must have an array value!"),
        };

        let instance = ListInstance(Rc::new(RefCell::new(ListData {
            model: model.clone(),
            children: None,
            parent,
        })));

        let mut children: HashMap<String, ListChildInstance> = HashMap::new();

        for list_value in value_arr.into_iter() {
            let children_parent = Rc::downgrade(&instance.0);
            let child_instance = ListChildInstance::new(model.clone(), list_value, children_parent);
            children.insert(child_instance.get_key(), child_instance);
        }

        instance.0.borrow_mut().children = Some(Rc::new(RefCell::new(children)));

        instance
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit)) {
        for child in self.0.borrow().children.as_ref().unwrap().borrow().values() {
            child.visit(f);
        }
    }
}

impl ListData {
    pub fn get_path(&self) -> String {
        let parent_path = match &self.parent {
            Parent::ContainerData(x) => x.upgrade().unwrap().borrow().get_path(),
            Parent::ListChildData(x) => x.upgrade().unwrap().borrow().get_path(),
        };

        format!("{}/{}", parent_path, self.model.name)
    }
}
