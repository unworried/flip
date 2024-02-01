use std::fmt::{self, Debug};

/// Owned Smart Pointer::: may not need this, inspired by rustc
#[derive(PartialEq)]
pub struct P<T: ?Sized> {
    pub ptr: Box<T>,
}

#[allow(non_snake_case)]
pub fn P<T: 'static>(value: T) -> P<T> {
    P {
        ptr: Box::new(value),
    }
}

impl<T: ?Sized + Debug> Debug for P<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.ptr, f)
    }
}
