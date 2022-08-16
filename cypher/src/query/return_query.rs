use std::sync::Arc;

use crate::node::Node;

use super::execute::{Execute, ExecuteTrait};

pub trait ReturnTrait: 'static {
    fn r#return(&self) -> Box<dyn ExecuteTrait>;
}

pub struct ReturnQuery {
    nv: String,
    state: String,
    data: Arc<Node>,
}

impl ReturnQuery {
    pub fn new(nv: String, state: String, data: Arc<Node>) -> Self {
        ReturnQuery { nv, state, data }
    }
}

impl ReturnTrait for ReturnQuery {
    fn r#return(&self) -> Box<dyn ExecuteTrait> {
        Box::new(Execute(self.state.clone()))
    }
}
