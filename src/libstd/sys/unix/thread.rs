// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

use prelude::v1::*;

use alloc::boxed::FnBox;
use io;
use libc;
use sys::os;
use time::Duration;

pub struct Thread;

// Some platforms may have pthread_t as a pointer in which case we still want
// a thread to be Send/Sync
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub unsafe fn new<'a>(_stack: usize, _p: Box<FnBox() + 'a>)
                          -> io::Result<Thread> {
        Ok(Thread)
    }

    pub fn yield_now() {
    }

    pub fn set_name(_name: &str) {
    }

    pub fn sleep(dur: Duration) {
        if dur < Duration::zero() {
            return Thread::yield_now()
        }
        let seconds = dur.num_seconds();
        let ns = dur - Duration::seconds(seconds);
        let mut ts = libc::timespec {
            tv_sec: seconds as libc::time_t,
            tv_nsec: ns.num_nanoseconds().unwrap() as libc::c_long,
        };

        // If we're awoken with a signal then the return value will be -1 and
        // nanosleep will fill in `ts` with the remaining time.
        unsafe {
            while libc::nanosleep(&ts, &mut ts) == -1 {
                assert_eq!(os::errno(), libc::EINTR);
            }
        }
    }

    pub fn join(self) {
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
    }
}

pub mod guard {
    pub unsafe fn current() -> usize { 0 }
    pub unsafe fn main() -> usize { 0 }
    pub unsafe fn init() {}
}
