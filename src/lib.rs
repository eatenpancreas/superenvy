pub use lazy_static::lazy_static as vars;
use std::env;

pub struct Var<'v> {
    pub inner: String,
    pub name: &'v str,
}

impl<'v> Var<'v> {
    #[inline]
    pub fn non_empty(self) -> Var<'v> {
        assert!(!self.inner.is_empty(), "{} must be non-empty", self.name);
        self
    }

    #[inline]
    pub fn bool(self) -> bool {
        match &*self.inner.to_lowercase() {
            "true" => true,
            "false" => false,
            _ => panic!("{} must be true or false", self.name),
        }
    }

    #[inline]
    pub fn new(name: &'v str) -> Self {
        dotenvy::dotenv().ok();
        Self {
            inner: env::var(name).expect(&format!("{name} must be set")),
            name,
        }
    }

    #[inline]
    pub fn new_opt(name: &'v str) -> Option<Self> {
        dotenvy::dotenv().ok();
        Some(Var {
            inner: env::var(name).ok()?,
            name,
        })
    }
}

#[inline]
pub fn var(name: &str) -> Var {
    Var::new(name)
}

#[inline]
pub fn var_opt(name: &str) -> Option<Var> {
    Var::new_opt(name)
}
