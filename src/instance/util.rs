use std::sync::{RwLock, Weak};

use super::containerinstance::{ContainerData, ContainerInstance};
use super::leafinstance::LeafInstance;
use super::leaflistinstance::LeafListInstance;
use super::listchildinstance::ListChildData;
use super::listinstance::ListInstance;

pub enum Child {
    ContainerInstance(ContainerInstance),
    LeafInstance(LeafInstance),
    LeafListInstance(LeafListInstance),
    ListInstance(ListInstance),
}

pub enum Parent {
    ContainerData(Weak<RwLock<ContainerData>>),
    ListChildData(Weak<RwLock<ListChildData>>),
}

pub enum NodeToVisit<'a> {
    LeafListInstance(&'a LeafListInstance),
    LeafInstance(&'a LeafInstance),
}

pub trait Generated {
    fn get_parent(&self) -> &Parent;

    fn is_generated(&self) -> bool {
        match &self.get_parent() {
            Parent::ListChildData(p) => p.upgrade().unwrap().read().unwrap().is_generated(),
            _ => false,
        }
    }
}
