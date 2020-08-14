use serde_json::Value;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use ustr::{ustr, UstrMap};

use super::leafinstance::LeafInstance;
use super::leaflistinstance::LeafListInstance;
use super::listinstance::ListInstance;
use super::util::*;
use crate::model::container::Container;
use crate::model::util::{Model, WithChildren};

pub struct ContainerData {
    pub parent: Option<Parent>,
    pub model: Arc<Container>,
    pub children: Option<Rc<RefCell<UstrMap<Child>>>>,
}

type Link = Rc<RefCell<ContainerData>>;

pub struct ContainerInstance(Link);

impl Clone for ContainerInstance {
    fn clone(&self) -> Self {
        ContainerInstance(Rc::clone(&self.0))
    }
}

impl PartialEq for ContainerInstance {
    fn eq(&self, other: &ContainerInstance) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

pub fn parse_children(model: Arc<Container>, value: Value, parent: &Link) -> UstrMap<Child> {
    let mut children: UstrMap<Child> = UstrMap::default();

    if let Value::Object(x) = value {
        for (k, v) in x.into_iter() {
            let child_model = model.get_child(&k).unwrap();
            let children_parent = Parent::ContainerData(Rc::downgrade(parent));

            match child_model {
                Model::Leaf(m) => {
                    children.insert(
                        ustr(&k),
                        Child::LeafInstance(LeafInstance::new(m.clone(), v, children_parent)),
                    );
                }
                Model::Container(m) => {
                    children.insert(
                        ustr(&k),
                        Child::ContainerInstance(ContainerInstance::new(
                            m.clone(),
                            v,
                            Some(children_parent),
                        )),
                    );
                }
                Model::LeafList(m) => {
                    children.insert(
                        ustr(&k),
                        Child::LeafListInstance(LeafListInstance::new(
                            m.clone(),
                            v,
                            children_parent,
                        )),
                    );
                }
                Model::List(m) => {
                    children.insert(
                        ustr(&k),
                        Child::ListInstance(ListInstance::new(m.clone(), v, children_parent)),
                    );
                }
            }
        }
    }

    children
}

impl ContainerInstance {
    pub fn new(model: Arc<Container>, value: Value, parent: Option<Parent>) -> ContainerInstance {
        let instance = ContainerInstance(Rc::new(RefCell::new(ContainerData {
            model: model.clone(),
            children: None,
            parent,
        })));

        instance.0.borrow_mut().children = Some(Rc::new(RefCell::new(parse_children(
            model,
            value,
            &instance.0,
        ))));

        instance
    }

    pub fn visit(&self, f: &dyn Fn(NodeToVisit) -> ()) {
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

impl ContainerData {
    pub fn get_path(&self) -> String {
        let parent_path = match &self.parent {
            Some(p) => match p {
                Parent::ContainerData(x) => x.upgrade().unwrap().borrow().get_path(),
                Parent::ListChildData(x) => x.upgrade().unwrap().borrow().get_path(),
            },
            None => "".to_string(),
        };

        format!("{}/{}", parent_path, self.model.name)
    }
}
