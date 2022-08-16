pub struct Execute(pub String);

pub trait ExecuteTrait: 'static {
    fn execute(&self) -> String;
}

impl ExecuteTrait for Execute {
    fn execute(&self) -> String {
        self.0.clone()
    }
}
