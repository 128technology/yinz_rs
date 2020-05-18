use std::cell::RefCell;
use std::rc::Weak;

use super::containerinstance::{ContainerData, ContainerInstance};
use super::leafinstance::LeafInstance;
use super::leaflistinstance::LeafListInstance;
use super::listchildinstance::ListChildData;
use super::listinstance::ListInstance;

pub enum Child<'a> {
    ContainerInstance(ContainerInstance<'a>),
    LeafInstance(LeafInstance<'a>),
    LeafListInstance(LeafListInstance<'a>),
    ListInstance(ListInstance<'a>),
}

pub enum Parent<'a> {
    ContainerData(Weak<RefCell<ContainerData<'a>>>),
    ListChildData(Weak<RefCell<ListChildData<'a>>>),
}
