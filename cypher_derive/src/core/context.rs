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

    pub fn error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T) {
        self.errs
            .borrow_mut()
            .as_mut()
            .unwrap()
            // Ограничение мономорфизацию от создания слишком большого количества идентичных методов
            .push(syn::Error::new_spanned(obj.into_token_stream(), msg));
    }

    /// Добавьте одну из ошибок разбора Syn
    pub fn syn_error(&self, err: syn::Error) {
        self.errs.borrow_mut().as_mut().unwrap().push(err);
    }
}
