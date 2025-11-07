#![no_std]

use core::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Canary<T> {
    type_id: TypeId,
    inner: T,
}

impl<T> Canary<T>
where
    T: 'static,
{
    pub const fn new(inner: T) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            inner,
        }
    }

    pub fn assert_valid(me: &Self) {
        assert!(
            me.type_id == TypeId::of::<T>(),
            "TypeId mismatch! {} is not properly initialized!",
            core::any::type_name::<T>(),
        );
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for Canary<T>
where
    T: 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        Self::assert_valid(self);

        &self.inner
    }
}

impl<T> DerefMut for Canary<T>
where
    T: 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[cfg(debug_assertions)]
        Self::assert_valid(self);

        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use core::mem::MaybeUninit;

    use super::*;

    #[test]
    fn basically_works() {
        let mut canary = Canary::<_>::new(42u64);
        assert_eq!(*canary, 42);
        *canary += 1;
        assert_eq!(*canary, 43);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn panics_with_incorrect_type() {
        let b = Canary::<_>::new(42f64);

        let b = unsafe { core::mem::transmute::<Canary<f64>, Canary<u64>>(b) };

        assert_eq!(*b, 42);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    #[cfg_attr(miri, ignore)]
    fn panics_with_uninitialized() {
        #[allow(invalid_value)]
        let b: Canary<u64> = unsafe { MaybeUninit::uninit().assume_init() };

        assert_eq!(*b, 42);
    }
}
