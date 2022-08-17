pub struct Finalize(pub String);

pub trait FinalizeTrait: 'static {
    fn finalize(&self) -> String;
}

impl FinalizeTrait for Finalize {
    fn finalize(&self) -> String {
        self.0.clone()
    }
}
