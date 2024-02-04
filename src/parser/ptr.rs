use alloc::boxed::Box;
use core::fmt::{Debug, Formatter, Result};
use std::{fmt::Display, ops::{Deref, DerefMut}};

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

impl<T: 'static> P<T> {
    pub fn into_inner(self) -> T {
        *self.ptr
    }
}

impl<T: ?Sized> Deref for P<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.ptr
    }
}

impl<T: ?Sized> DerefMut for P<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ptr
    }
}

impl<T: 'static + Clone> Clone for P<T> {
    fn clone(&self) -> P<T> {
        P((**self).clone())
    }
}

impl<T: ?Sized + Debug> Debug for P<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.ptr, f)
    }
}

impl<T: Display> Display for P<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&**self, f)
    }
}
