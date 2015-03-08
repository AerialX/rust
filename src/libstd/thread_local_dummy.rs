// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Thread local unstorage
//!
//! This module provides no implementation of thread local storage for Rust
//! programs.

#![stable(feature = "rust1", since = "1.0.0")]
#![doc(hidden)]

use prelude::v1::*;

use cell::UnsafeCell;

#[macro_use]
#[unstable(feature = "std_misc",
            reason = "scoped TLS has yet to have wide enough use to fully consider \
                      stabilizing its interface")]
pub mod scoped {
    use prelude::v1::*;
    use cell::UnsafeCell;

    pub struct Key<T> { #[doc(hidden)] pub inner: UnsafeCell<*mut T> }
    unsafe impl<T> ::marker::Sync for Key<T> { }

    #[macro_export]
    #[allow_internal_unstable]
    macro_rules! scoped_thread_local {
        (static $name:ident: $t:ty) => (
            use std::thread_local::scoped::Key as __Key;
            static $name: __Key<$t> = __Key {
                inner: ::std::cell::UnsafeCell { value: 0 as *mut _ },
            }
            );
        (pub static $name:ident: $t:ty) => (
            use std::thread_local::scoped::Key as __Key;
            pub static $name: __Key<$t> = __Key {
                inner: ::std::cell::UnsafeCell { value: 0 as *mut _ },
            }
            );
    }

    impl<T> Key<T> {
        pub fn set<R, F>(&'static self, t: &T, cb: F) -> R where F: FnOnce() -> R, {
            unsafe {
                *self.inner.get() = t as *const T as *mut T;
            }
            cb()
        }

        pub fn with<R, F>(&'static self, cb: F) -> R where F: FnOnce(&T) -> R {
            unsafe {
                let ptr = *self.inner.get();
                assert!(!ptr.is_null(), "cannot access a scoped thread local \
                                     variable without calling `set` first");
                cb(&*ptr)
            }
        }

        pub fn is_set(&'static self) -> bool {
            unsafe { !(*self.inner.get()).is_null() }
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct Key<T> {
    #[doc(hidden)]
    #[unstable(feature = "thread_local_internals")]
    pub inner: UnsafeCell<Option<T>>,

    #[doc(hidden)]
    #[unstable(feature = "thread_local_internals")]
    pub init: fn() -> T,
}

unsafe impl<T> ::marker::Sync for Key<T> { }

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow_internal_unstable]
macro_rules! thread_local {
    (static $name:ident: $t:ty = $init:expr) => (
        static $name: ::std::thread_local::Key<$t> = {
            use std::cell::UnsafeCell as __UnsafeCell;
            use std::option::Option::None as __None;

            fn __init() -> $t { $init }
            ::std::thread_local::Key { inner: __UnsafeCell { value: __None }, init: __init }
        };
        );
    (pub static $name:ident: $t:ty = $init:expr) => (
        pub static $name: ::std::thread_local::Key<$t> = {
            use std::cell::UnsafeCell as __UnsafeCell;
            use std::option::Option::None as __None;

            fn __init() -> $t { $init }
            ::std::thread_local::Key { inner: __UnsafeCell { value: __None }, init: __init }
        };
        );
}

#[unstable(feature = "std_misc",
           reason = "state querying was recently added")]
#[derive(Eq, PartialEq, Copy)]
pub enum State {
    Uninitialized,
    Valid,
    Destroyed,
}

impl<T: 'static> Key<T> {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn with<F, R>(&'static self, f: F) -> R
        where F: FnOnce(&T) -> R {
            unsafe {
                f(match *self.inner.get() {
                    Some(ref inner) => inner,
                    None => self.init(),
                })
            }
        }

    unsafe fn init(&self) -> &T {
        let value = (self.init)();
        let ptr = self.inner.get();
        *ptr = Some(value);
        (*ptr).as_ref().unwrap()
    }

    #[unstable(feature = "std_misc",
               reason = "state querying was recently added")]
        pub fn state(&'static self) -> State {
            unsafe {
                match *self.inner.get() {
                    Some(..) => State::Valid,
                    None => State::Uninitialized,
                }
            }
        }

    #[unstable(feature = "std_misc")]
    #[deprecated(since = "1.0.0",
                 reason = "function renamed to state() and returns more info")]
        pub fn destroyed(&'static self) -> bool { self.state() == State::Destroyed }
}
