// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! No threads

#![stable(feature = "rust1", since = "1.0.0")]
#![doc(hidden)]

use prelude::v1::*;

use any::Any;
use fmt;
use rt::unwind;
use time::Duration;
use sys_common::thread_info;

#[stable(feature = "rust1", since = "1.0.0")]
pub fn current() -> Thread {
    Thread
}

#[stable(feature = "rust1", since = "1.0.0")]
pub fn yield_now() {

}

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn panicking() -> bool {
    unwind::panicking()
}

#[stable(feature = "rust1", since = "1.0.0")]
pub fn park() {

}

#[unstable(feature = "std_misc", reason = "recently introduced, depends on Duration")]
pub fn park_timeout(_: Duration) {

}

#[derive(Clone)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Thread;

impl Thread {
    #[deprecated(since = "1.0.0", reason = "use module-level free function")]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn current() -> Thread {
        Thread
    }

    #[deprecated(since = "1.0.0", reason = "use module-level free function")]
    #[unstable(feature = "std_misc", reason = "name may change")]
    pub fn yield_now() {

    }

    #[deprecated(since = "1.0.0", reason = "use module-level free function")]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn panicking() -> bool {
        unwind::panicking()
    }

    #[deprecated(since = "1.0.0", reason = "use module-level free function")]
    #[unstable(feature = "std_misc", reason = "recently introduced")]
    pub fn park() {

    }

    /// Deprecated: use module-level free function.
    #[deprecated(since = "1.0.0", reason = "use module-level free function")]
    #[unstable(feature = "std_misc", reason = "recently introduced")]
    pub fn park_timeout(_: Duration) {

    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unpark(&self) {

    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn name(&self) -> Option<&str> {
        Some("main")
    }
}

impl thread_info::NewThread for Thread {
    fn new(_: Option<String>) -> Thread { Thread }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.name(), f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub type Result<T> = ::result::Result<T, Box<Any + Send + 'static>>;
