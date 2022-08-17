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
        self.state.push_str(&format!(" RETURN {}", self.nv));
        Box::new(Execute(self.state.clone()))
    }

    fn return_as(&mut self, r#as: &str) -> Box<dyn ExecuteTrait> {
        self.state
            .push_str(&format!(" RETURN {} AS {}", self.nv, r#as));
        Box::new(Execute(self.state.clone()))
    }

    fn return_field(&mut self, field: &str) -> Box<dyn ExecuteTrait> {
        self.state
            .push_str(&format!(" RETURN {}.{}", self.nv, field));
        Box::new(Execute(self.state.clone()))
    }
}
