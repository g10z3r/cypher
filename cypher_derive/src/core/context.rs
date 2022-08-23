use quote::ToTokens;
use std::cell::RefCell;
use std::fmt::Display;
use std::thread;
use syn;

pub struct Context {
    errs: RefCell<Option<Vec<syn::Error>>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            errs: RefCell::new(Some(Vec::new())),
        }
    }

    /// Add an error to the context object with a tokenenizable object.
    ///
    /// The object is used for spanning in error messages.
    pub fn error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T) {
        self.errs
            .borrow_mut()
            .as_mut()
            .unwrap()
            // Ограничение мономорфизацию от создания слишком большого количества идентичных методов
            .push(syn::Error::new_spanned(obj.into_token_stream(), msg));
    }

    /// Add one of Syn's parse errors.
    pub fn syn_error(&self, err: syn::Error) {
        self.errs.borrow_mut().as_mut().unwrap().push(err);
    }

    /// Consume this object, producing a formatted error string if there are errors.
    pub fn check(&self) -> Result<(), Vec<syn::Error>> {
        let errors = self.errs.borrow_mut().take().unwrap();
        match errors.len() {
            0 => Ok(()),
            _ => Err(errors),
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if !thread::panicking() && self.errs.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}
