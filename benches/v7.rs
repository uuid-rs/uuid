#![cfg(all(feature = "v7", feature = "std"))]
#![feature(test)]
extern crate test;

use test::Bencher;
use uuid::Uuid;

#[bench]
fn now_v7(b: &mut Bencher) {
    b.iter(|| Uuid::now_v7());
}
