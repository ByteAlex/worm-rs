pub use worm_macro::rest;

pub mod nullable {
    pub use rest_nullable::{self, Nullable, deserialize_optional_nullable};
}

#[macro_export]
macro_rules! apply_some {
    ($obj:ident, $field:ident $(=)? $stmt:expr) => {
        if let Some(value) = $stmt {
            paste::paste! {
                $obj.[<set_ $field >](value.into());
            }
        }
    };
}

pub trait Apply<F> {
    fn apply(&mut self, from: F);
}

pub trait CloneInto<T> {
    fn clone_into(&self) -> T;
}