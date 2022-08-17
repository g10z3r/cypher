use super::execute::{Execute, ExecuteTrait};

pub trait ReturnTrait: 'static {
    fn r#return(&mut self) -> Box<dyn ExecuteTrait>;
    fn return_as(&mut self, r#as: &str) -> Box<dyn ExecuteTrait>;
    fn return_field(&mut self, field: &str) -> Box<dyn ExecuteTrait>;
}

pub struct ReturnQuery {
    nv: String,
    state: String,
}

impl ReturnQuery {
    pub fn new(nv: String, state: String) -> Self {
        ReturnQuery { nv, state }
    }
}

impl ReturnTrait for ReturnQuery {
    fn r#return(&mut self) -> Box<dyn ExecuteTrait> {
        let state = format!("{}\nRETURN {}", self.state, self.nv);
        Box::new(Execute(state))
    }

    fn return_as(&mut self, r#as: &str) -> Box<dyn ExecuteTrait> {
        let state = format!("{}\nRETURN {} AS {}", self.state, self.nv, r#as);
        Box::new(Execute(state))
    }

    fn return_field(&mut self, field: &str) -> Box<dyn ExecuteTrait> {
        let state = format!("{}\nRETURN {}.{}", self.state, self.nv, field);
        Box::new(Execute(state))
    }
}
