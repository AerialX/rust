// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unimplementation of Rust stack unwinding
//!
//! Does absolutely nothing

use prelude::v1::*;

use any::Any;
use cell::Cell;
use cmp;
use panicking;
use fmt;
use intrinsics;
use mem;
use sync::atomic::{self, Ordering};
use sync::{Once, ONCE_INIT};

pub type Callback = fn(msg: &(Any + Send), file: &'static str, line: uint);

// Variables used for invoking callbacks when a thread starts to unwind.
//
// For more information, see below.
const MAX_CALLBACKS: uint = 16;
static CALLBACKS: [atomic::AtomicUsize; MAX_CALLBACKS] =
        [atomic::ATOMIC_USIZE_INIT, atomic::ATOMIC_USIZE_INIT,
         atomic::ATOMIC_USIZE_INIT, atomic::ATOMIC_USIZE_INIT,
         atomic::ATOMIC_USIZE_INIT, atomic::ATOMIC_USIZE_INIT,
         atomic::ATOMIC_USIZE_INIT, atomic::ATOMIC_USIZE_INIT,
         atomic::ATOMIC_USIZE_INIT, atomic::ATOMIC_USIZE_INIT,
         atomic::ATOMIC_USIZE_INIT, atomic::ATOMIC_USIZE_INIT,
         atomic::ATOMIC_USIZE_INIT, atomic::ATOMIC_USIZE_INIT,
         atomic::ATOMIC_USIZE_INIT, atomic::ATOMIC_USIZE_INIT];
static CALLBACK_CNT: atomic::AtomicUsize = atomic::ATOMIC_USIZE_INIT;

thread_local! { static PANICKING: Cell<bool> = Cell::new(false) }

pub unsafe fn try<F: FnOnce()>(f: F) -> Result<(), Box<Any + Send>> {
    let prev = PANICKING.with(|s| s.get());
    PANICKING.with(|s| s.set(false));
    f();
    PANICKING.with(|s| s.set(prev));
    Ok(())
}

pub fn panicking() -> bool {
    PANICKING.with(|s| s.get())
}

#[inline(never)]
#[no_mangle]
#[allow(private_no_mangle_fns)]
fn rust_panic(cause: Box<Any + Send + 'static>) -> ! {
    let msg = match cause.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match cause.downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        }
    };
    rtabort!("panic: {}", msg);
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(msg: fmt::Arguments,
                                file: &'static str, line: uint) -> ! {
    begin_unwind_fmt(msg, &(file, line))
}

#[inline(never)] #[cold]
#[stable(since = "1.0.0", feature = "rust1")]
pub fn begin_unwind_fmt(msg: fmt::Arguments, file_line: &(&'static str, uint)) -> ! {
    use fmt::Write;

    let mut s = String::new();
    let _ = write!(&mut s, "{}", msg);
    begin_unwind_inner(Box::new(s), file_line)
}

#[inline(never)] #[cold]
#[stable(since = "1.0.0", feature = "rust1")]
pub fn begin_unwind<M: Any + Send>(msg: M, file_line: &(&'static str, uint)) -> ! {
    begin_unwind_inner(Box::new(msg), file_line)
}

#[inline(never)] #[cold]
fn begin_unwind_inner(msg: Box<Any + Send>, file_line: &(&'static str, uint)) -> ! {
    static INIT: Once = ONCE_INIT;
    INIT.call_once(|| unsafe { register(panicking::on_panic); });

    let callbacks = {
        let amt = CALLBACK_CNT.load(Ordering::SeqCst);
        &CALLBACKS[..cmp::min(amt, MAX_CALLBACKS)]
    };
    for cb in callbacks {
        match cb.load(Ordering::SeqCst) {
            0 => {}
            n => {
                let f: Callback = unsafe { mem::transmute(n) };
                let (file, line) = *file_line;
                f(&*msg, file, line);
            }
        }
    };

    if panicking() {
        rterrln!("thread panicked while panicking. aborting.");
        unsafe { intrinsics::abort() }
    }
    PANICKING.with(|s| s.set(true));
    rust_panic(msg);
}

#[unstable(feature = "std_misc")]
pub unsafe fn register(f: Callback) -> bool {
    match CALLBACK_CNT.fetch_add(1, Ordering::SeqCst) {
        n if n < MAX_CALLBACKS => {
            let prev = CALLBACKS[n].swap(mem::transmute(f), Ordering::SeqCst);
            rtassert!(prev == 0);
            true
        }
        _ => {
            CALLBACK_CNT.store(MAX_CALLBACKS, Ordering::SeqCst);
            false
        }
    }
}

#[doc(hidden)]
#[lang="eh_personality"]
#[no_mangle]
#[allow(private_no_mangle_fns)]
extern fn rust_eh_personality() -> !
{
    unimplemented!();
}

#[doc(hidden)]
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn _Unwind_Resume() -> !
{
    unimplemented!();
}
