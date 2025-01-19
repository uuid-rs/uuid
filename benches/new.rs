#![feature(test)]
extern crate test;

use test::Bencher;
use uuid::Uuid;

#[bench]
fn from_bytes(b: &mut Bencher) {
    b.iter(|| Uuid::from_bytes([
        0xF9, 0x16, 0x8C, 0x5E, 0xCE, 0xB2, 0x4F, 0xAA, 0xB6, 0xBF, 0x32, 0x9B, 0xF3, 0x9F,
        0xA1, 0xE4,
    ]));
}

#[bench]
fn from_u128(b: &mut Bencher) {
    b.iter(|| Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8));
}

#[bench]
fn from_u64_pair(b: &mut Bencher) {
    b.iter(|| Uuid::from_u64_pair(0xa1a2a3a4b1b2c1c2, 0xd1d2d3d4d5d6d7d8));
}
