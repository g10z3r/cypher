pub struct Finalize(pub String);

pub trait FinalizeTrait: 'static {
    /// End the query building process and return a string.
    fn finalize(&self) -> String;
}

impl FinalizeTrait for Finalize {
    fn finalize(&self) -> String {
        self.0.clone()
    }
}
