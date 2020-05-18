use serde_json::Value;

use super::util::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::leafinstance::LeafInstance;
use super::listchildinstance::ListChildInstance;
use crate::model::list::List;

pub struct ListData<'a> {
    pub parent: Parent<'a>,
    pub model: &'a List,
    pub children: Option<Rc<RefCell<HashMap<String, ListChildInstance<'a>>>>>,
    pub path: String,
}

type Link<'a> = Rc<RefCell<ListData<'a>>>;

pub struct ListInstance<'a>(Link<'a>);

impl<'a> Clone for ListInstance<'a> {
    fn clone(&self) -> Self {
        ListInstance(Rc::clone(&self.0))
    }
}

impl<'a> PartialEq for ListInstance<'a> {
    fn eq(&self, other: &ListInstance<'a>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<'a> ListInstance<'a> {
    pub fn new(
        model: &'a List,
        value: &Value,
        parent_path: String,
        parent: Parent<'a>,
    ) -> ListInstance<'a> {
        let value_arr = match value {
            Value::Array(x) => x,
            _ => panic!("List must have an array value!"),
        };

        let path = format!("{}/{}", parent_path, model.name);
        let child_path = path.clone();

        let instance = ListInstance(Rc::new(RefCell::new(ListData {
            model,
            children: None,
            path,
            parent,
        })));

        let mut children: HashMap<String, ListChildInstance> = HashMap::new();

        for list_value in value_arr.iter() {
            let children_parent = Rc::downgrade(&instance.0);
            let child_instance =
                ListChildInstance::new(model, list_value, child_path.clone(), children_parent);
            children.insert(child_instance.get_key(), child_instance);
        }

        instance.0.borrow_mut().children = Some(Rc::new(RefCell::new(children)));

        instance
    }

    pub fn visit(&self, f: &dyn Fn(&LeafInstance) -> ()) {
        for child in self.0.borrow().children.as_ref().unwrap().borrow().values() {
            child.visit(f);
        }
    }
}
