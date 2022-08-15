#[macro_export]
macro_rules! generate_query {
    () => {
        #[derive(Clone)]
        pub struct Query<T> {
            /// Variable of current node.
            /// Default is `n`
            nv: String,
            /// The current state of the request
            state: String,

            source: T,
        }

        impl<T> Query<T> {
            pub fn new(source: T) -> Self {
                Self {
                    nv: String::from("n"),
                    state: String::new(),
                    source,
                }
            }

            pub fn state(&self) -> String {
                self.state.clone()
            }

            pub fn push_to_state(&mut self, ns: &str) {
                self.state.push_str(ns);
            }

            pub fn nv(&self) -> String {
                self.nv.clone()
            }

            pub fn set_nv(&mut self, name: &str) {
                self.nv = String::from(name);
            }
        }
    };
}
