use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::leafinstance::LeafInstance;
use super::leaflistinstance::LeafListInstance;
use super::listinstance::ListInstance;
use super::util::*;
use crate::model::container::Container;
use crate::model::util::Model;

pub struct ContainerData<'a> {
    pub parent: Option<Parent<'a>>,
    pub model: &'a Container,
    pub children: Option<Rc<RefCell<HashMap<String, Child<'a>>>>>,
    pub path: String,
}

type Link<'a> = Rc<RefCell<ContainerData<'a>>>;

pub struct ContainerInstance<'a>(Link<'a>);

impl<'a> Clone for ContainerInstance<'a> {
    fn clone(&self) -> Self {
        ContainerInstance(Rc::clone(&self.0))
    }
}

impl<'a> PartialEq for ContainerInstance<'a> {
    fn eq(&self, other: &ContainerInstance<'a>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

pub fn parse_children<'a>(
    model: &'a Container,
    value: &Value,
    parent_path: String,
    parent: &Link<'a>,
) -> HashMap<String, Child<'a>> {
    let mut children: HashMap<String, Child> = HashMap::new();
    // TODO: Remove these clones
    let child_path = parent_path.clone();

    match value {
        Value::Object(x) => {
            for (k, v) in x.iter() {
                let child_model = model.children.get(k).unwrap();
                let children_parent = Parent::ContainerData(Rc::downgrade(parent));

                match child_model {
                    Model::Leaf(m) => {
                        children.insert(
                            k.to_string(),
                            Child::LeafInstance(LeafInstance::new(
                                m,
                                &v,
                                child_path.clone(),
                                children_parent,
                            )),
                        );
                    }
                    Model::Container(m) => {
                        children.insert(
                            k.to_string(),
                            Child::ContainerInstance(ContainerInstance::new(
                                m,
                                &v,
                                child_path.clone(),
                                Some(children_parent),
                            )),
                        );
                    }
                    Model::LeafList(m) => {
                        children.insert(
                            k.to_string(),
                            Child::LeafListInstance(LeafListInstance::new(
                                m,
                                &v,
                                child_path.clone(),
                                children_parent,
                            )),
                        );
                    }
                    Model::List(m) => {
                        children.insert(
                            k.to_string(),
                            Child::ListInstance(ListInstance::new(
                                m,
                                &v,
                                child_path.clone(),
                                children_parent,
                            )),
                        );
                    }
                }
            }
        }
        _ => {}
    }

    return children;
}

impl<'a> ContainerInstance<'a> {
    pub fn new(
        model: &'a Container,
        value: &Value,
        parent_path: String,
        parent: Option<Parent<'a>>,
    ) -> ContainerInstance<'a> {
        let path = format!("{}/{}", parent_path, model.name);
        let child_path = path.clone();

        let instance = ContainerInstance(Rc::new(RefCell::new(ContainerData {
            model,
            children: None,
            path,
            parent,
        })));

        instance.0.borrow_mut().children = Some(Rc::new(RefCell::new(parse_children(
            model,
            value,
            child_path,
            &instance.0,
        ))));

        instance
    }

    pub fn visit(&self, f: &dyn Fn(&LeafInstance) -> ()) {
        for child in self.0.borrow().children.as_ref().unwrap().borrow().values() {
            match child {
                Child::ContainerInstance(c) => {
                    c.visit(f);
                }
                Child::ListInstance(c) => {
                    c.visit(f);
                }
                Child::LeafInstance(c) => {
                    c.visit(f);
                }
                Child::LeafListInstance(c) => {
                    c.visit(f);
                }
            }
        }
    }
}
