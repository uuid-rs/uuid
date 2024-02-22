#![feature(test)]

extern crate test;

use test::Bencher;

#[cfg(feature = "v1")]
#[bench]
fn bench_v1(b: &mut Bencher) {
    b.iter(|| {
        let node_id: [u8; 6] = [1, 2, 3, 4, 5, 6];
        let uuid = uuid::Uuid::now_v1(&node_id);
    })
}

#[cfg(feature = "v1_auto")]
#[bench]
fn bench_v1_auto(b: &mut Bencher) {
    let uuid = uuid::Uuid::now_v1_auto();
    b.iter(|| {
        let uuid = uuid::Uuid::now_v1_auto();
    })
}
