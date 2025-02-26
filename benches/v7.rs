#![cfg(all(feature = "v7", feature = "std"))]
#![feature(test)]
extern crate test;

use std::time::SystemTime;

use test::Bencher;
use uuid::{ContextV7, NoContext, Timestamp, Uuid};

#[bench]
fn now_v7(b: &mut Bencher) {
    b.iter(|| Uuid::now_v7());
}

#[bench]
fn new_v7_no_context(b: &mut Bencher) {
    b.iter(|| Uuid::new_v7(Timestamp::now(NoContext)));
}

#[bench]
fn new_v7_context(b: &mut Bencher) {
    let ctxt = ContextV7::new();

    b.iter(|| Uuid::new_v7(Timestamp::now(&ctxt)));
}

#[bench]
fn new_v7_context_additional_precision(b: &mut Bencher) {
    let ctxt = ContextV7::new().with_additional_precision();

    b.iter(|| Uuid::new_v7(Timestamp::now(&ctxt)));
}

#[bench]
fn v7_raw(b: &mut Bencher) {
    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap();
    let secs = now.as_secs();
    let subsec_nanos = now.subsec_nanos();
    let mut counter = 0;

    b.iter(|| {
        Uuid::new_v7(Timestamp::from_unix_time(
            secs,
            subsec_nanos,
            {
                counter += 1;
                counter
            },
            42,
        ))
    });
}
