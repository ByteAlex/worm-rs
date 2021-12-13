pub use worm_macro::{rest, rest_object};
pub use paste::paste;

pub mod nullable {
    pub use rest_nullable::{self, Nullable, deserialize_optional_nullable};
}

#[macro_export]
macro_rules! apply_some {
    ($obj:ident, $field:ident $(=)? $stmt:expr) => {
        if let Some(value) = $stmt {
            worm::paste! {
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