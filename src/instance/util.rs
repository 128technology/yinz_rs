use std::sync::{RwLock, Weak};

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
    ContainerData(Weak<RwLock<ContainerData<'a>>>),
    ListChildData(Weak<RwLock<ListChildData<'a>>>),
}
