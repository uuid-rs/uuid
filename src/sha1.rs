use core::ops::{Add, BitXor};

const K0: u32 = 0x5A827999u32;
const K1: u32 = 0x6ED9EBA1u32;
const K2: u32 = 0x8F1BBCDCu32;
const K3: u32 = 0xCA62C1D6u32;

const H: [u32; 5] = [
    0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0
];

const BLOCK_SIZE: usize = 64;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
struct u32x4(u32, u32, u32, u32);

impl BitXor for u32x4 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        u32x4(self.0 ^ rhs.0, self.1 ^ rhs.1, self.2 ^ rhs.2, self.3 ^ rhs.3)
    }
}

impl Add for u32x4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        u32x4(
            self.0.wrapping_add(rhs.0),
            self.1.wrapping_add(rhs.1),
            self.2.wrapping_add(rhs.2),
            self.3.wrapping_add(rhs.3),
        )
    }
}

#[inline]
fn sha1_first(w0: u32x4) -> u32 {
    w0.0
}

#[inline]
fn sha1_first_add(e: u32, w0: u32x4) -> u32x4 {
    let u32x4(a, b, c, d) = w0;
    u32x4(e.wrapping_add(a), b, c, d)
}

fn sha1msg1(a: u32x4, b: u32x4) -> u32x4 {
    let u32x4(_, _, w2, w3) = a;
    let u32x4(w4, w5, _, _) = b;
    a ^ u32x4(w2, w3, w4, w5)
}

fn sha1msg2(a: u32x4, b: u32x4) -> u32x4 {
    let u32x4(x0, x1, x2, x3) = a;
    let u32x4(_, w13, w14, w15) = b;

    let w16 = (x0 ^ w13).rotate_left(1);
    let w17 = (x1 ^ w14).rotate_left(1);
    let w18 = (x2 ^ w15).rotate_left(1);
    let w19 = (x3 ^ w16).rotate_left(1);

    u32x4(w16, w17, w18, w19)
}

#[inline]
fn sha1_first_half(abcd: u32x4, msg: u32x4) -> u32x4 {
    sha1_first_add(sha1_first(abcd).rotate_left(30), msg)
}

fn sha1_digest_round_x4(abcd: u32x4, work: u32x4, i: i8) -> u32x4 {
    const K0V: u32x4 = u32x4(K0, K0, K0, K0);
    const K1V: u32x4 = u32x4(K1, K1, K1, K1);
    const K2V: u32x4 = u32x4(K2, K2, K2, K2);
    const K3V: u32x4 = u32x4(K3, K3, K3, K3);

    match i {
        0 => sha1rnds4c(abcd, work + K0V),
        1 => sha1rnds4p(abcd, work + K1V),
        2 => sha1rnds4m(abcd, work + K2V),
        3 => sha1rnds4p(abcd, work + K3V),
        _ => unreachable!("unknown icosaround index"),
    }
}

fn sha1rnds4c(abcd: u32x4, msg: u32x4) -> u32x4 {
    let u32x4(mut a, mut b, mut c, mut d) = abcd;
    let u32x4(t, u, v, w) = msg;
    let mut e = 0u32;

    macro_rules! bool3ary_202 {
        ($a:expr, $b:expr, $c:expr) => (($c ^ ($a & ($b ^ $c))))
    } // Choose, MD5F, SHA1C

    e = e.wrapping_add(a.rotate_left(5))
        .wrapping_add(bool3ary_202!(b, c, d))
        .wrapping_add(t);
    b = b.rotate_left(30);

    d = d.wrapping_add(e.rotate_left(5))
        .wrapping_add(bool3ary_202!(a, b, c))
        .wrapping_add(u);
    a = a.rotate_left(30);

    c = c.wrapping_add(d.rotate_left(5))
        .wrapping_add(bool3ary_202!(e, a, b))
        .wrapping_add(v);
    e = e.rotate_left(30);

    b = b.wrapping_add(c.rotate_left(5))
        .wrapping_add(bool3ary_202!(d, e, a))
        .wrapping_add(w);
    d = d.rotate_left(30);

    u32x4(b, c, d, e)
}

fn sha1rnds4p(abcd: u32x4, msg: u32x4) -> u32x4 {
    let u32x4(mut a, mut b, mut c, mut d) = abcd;
    let u32x4(t, u, v, w) = msg;
    let mut e = 0u32;

    macro_rules! bool3ary_150 {
        ($a:expr, $b:expr, $c:expr) => (($a ^ $b ^ $c))
    } // Parity, XOR, MD5H, SHA1P

    e = e.wrapping_add(a.rotate_left(5))
        .wrapping_add(bool3ary_150!(b, c, d))
        .wrapping_add(t);
    b = b.rotate_left(30);

    d = d.wrapping_add(e.rotate_left(5))
        .wrapping_add(bool3ary_150!(a, b, c))
        .wrapping_add(u);
    a = a.rotate_left(30);

    c = c.wrapping_add(d.rotate_left(5))
        .wrapping_add(bool3ary_150!(e, a, b))
        .wrapping_add(v);
    e = e.rotate_left(30);

    b = b.wrapping_add(c.rotate_left(5))
        .wrapping_add(bool3ary_150!(d, e, a))
        .wrapping_add(w);
    d = d.rotate_left(30);

    u32x4(b, c, d, e)
}

fn sha1rnds4m(abcd: u32x4, msg: u32x4) -> u32x4 {
    let u32x4(mut a, mut b, mut c, mut d) = abcd;
    let u32x4(t, u, v, w) = msg;
    let mut e = 0u32;

    macro_rules! bool3ary_232 {
        ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ ($a & $c) ^ ($b & $c))
    } // Majority, SHA1M

    e = e.wrapping_add(a.rotate_left(5))
        .wrapping_add(bool3ary_232!(b, c, d))
        .wrapping_add(t);
    b = b.rotate_left(30);

    d = d.wrapping_add(e.rotate_left(5))
        .wrapping_add(bool3ary_232!(a, b, c))
        .wrapping_add(u);
    a = a.rotate_left(30);

    c = c.wrapping_add(d.rotate_left(5))
        .wrapping_add(bool3ary_232!(e, a, b))
        .wrapping_add(v);
    e = e.rotate_left(30);

    b = b.wrapping_add(c.rotate_left(5))
        .wrapping_add(bool3ary_232!(d, e, a))
        .wrapping_add(w);
    d = d.rotate_left(30);

    u32x4(b, c, d, e)
}

fn sha1_digest_block_u32(state: &mut [u32; 5], block: &[u32; 16]) {

    macro_rules! schedule {
        ($v0:expr, $v1:expr, $v2:expr, $v3:expr) => (
            sha1msg2(sha1msg1($v0, $v1) ^ $v2, $v3)
        )
    }

    macro_rules! rounds4 {
        ($h0:ident, $h1:ident, $wk:expr, $i:expr) => (
            sha1_digest_round_x4($h0, sha1_first_half($h1, $wk), $i)
        )
    }

    // Rounds 0..20
    let mut h0 = u32x4(state[0], state[1], state[2], state[3]);
    let mut w0 = u32x4(block[0], block[1], block[2], block[3]);
    let mut h1 = sha1_digest_round_x4(h0, sha1_first_add(state[4], w0), 0);
    let mut w1 = u32x4(block[4], block[5], block[6], block[7]);
    h0 = rounds4!(h1, h0, w1, 0);
    let mut w2 = u32x4(block[8], block[9], block[10], block[11]);
    h1 = rounds4!(h0, h1, w2, 0);
    let mut w3 = u32x4(block[12], block[13], block[14], block[15]);
    h0 = rounds4!(h1, h0, w3, 0);
    let mut w4 = schedule!(w0, w1, w2, w3);
    h1 = rounds4!(h0, h1, w4, 0);

    // Rounds 20..40
    w0 = schedule!(w1, w2, w3, w4);
    h0 = rounds4!(h1, h0, w0, 1);
    w1 = schedule!(w2, w3, w4, w0);
    h1 = rounds4!(h0, h1, w1, 1);
    w2 = schedule!(w3, w4, w0, w1);
    h0 = rounds4!(h1, h0, w2, 1);
    w3 = schedule!(w4, w0, w1, w2);
    h1 = rounds4!(h0, h1, w3, 1);
    w4 = schedule!(w0, w1, w2, w3);
    h0 = rounds4!(h1, h0, w4, 1);

    // Rounds 40..60
    w0 = schedule!(w1, w2, w3, w4);
    h1 = rounds4!(h0, h1, w0, 2);
    w1 = schedule!(w2, w3, w4, w0);
    h0 = rounds4!(h1, h0, w1, 2);
    w2 = schedule!(w3, w4, w0, w1);
    h1 = rounds4!(h0, h1, w2, 2);
    w3 = schedule!(w4, w0, w1, w2);
    h0 = rounds4!(h1, h0, w3, 2);
    w4 = schedule!(w0, w1, w2, w3);
    h1 = rounds4!(h0, h1, w4, 2);

    // Rounds 60..80
    w0 = schedule!(w1, w2, w3, w4);
    h0 = rounds4!(h1, h0, w0, 3);
    w1 = schedule!(w2, w3, w4, w0);
    h1 = rounds4!(h0, h1, w1, 3);
    w2 = schedule!(w3, w4, w0, w1);
    h0 = rounds4!(h1, h0, w2, 3);
    w3 = schedule!(w4, w0, w1, w2);
    h1 = rounds4!(h0, h1, w3, 3);
    w4 = schedule!(w0, w1, w2, w3);
    h0 = rounds4!(h1, h0, w4, 3);

    let e = sha1_first(h1).rotate_left(30);
    let u32x4(a, b, c, d) = h0;

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
}

fn compress(state: &mut [u32; 5], block: &[u8]) {
    assert_eq!(block.len(), 64);
    let mut block_u32 = [0u32; 16];

    for i in 0..16 {
        let off = i * 4;
        block_u32[i] = (block[off + 3] as u32) |
                       ((block[off + 2] as u32) << 8) |
                       ((block[off + 1] as u32) << 16) |
                       ((block[off] as u32) << 24);
    }

    sha1_digest_block_u32(state, &block_u32);
}

/// Compute SHA1 hash of concatenated namespace and name
pub fn compute(namespace: &[u8; 16], name: &str) -> [u8; 16] {
    let size = (((namespace.len() as u64) + (name.len() as u64)) << 3).to_be();
    let mut name = name.as_bytes();
    let mut state = H;
    let mut block = [0u8; BLOCK_SIZE];
    let mut pos = 16;

    block[..pos].copy_from_slice(namespace);

    if pos + name.len() < BLOCK_SIZE {
        block[pos..pos+name.len()].copy_from_slice(name);
        pos += name.len();
    } else {
        let (l, r) = name.split_at(BLOCK_SIZE - pos);
        name = r;
        block[pos..].copy_from_slice(l);
        compress(&mut state, &block);

        while name.len() >= BLOCK_SIZE {
            let (l, r) = name.split_at(BLOCK_SIZE);
            name = r;
            compress(&mut state, l);
        }

        block = [0u8; BLOCK_SIZE];
        pos = name.len();
        block[..pos].copy_from_slice(name);
    }

    block[pos] = 0x80;
    pos += 1;

    if BLOCK_SIZE - pos < 8 {
        compress(&mut state, &block);
        block = [0u8; BLOCK_SIZE];
    }

    for i in 0..8 {
        block[56+i] = (size >> (8*i)) as u8;
    }
    compress(&mut state, &block);

    let mut res = [0u8; 16];
    for i in 0..4 {
        res[4*i] = (state[i] >> 24) as u8;
        res[4*i + 1] = (state[i] >> 16) as u8;
        res[4*i + 2] = (state[i] >> 8) as u8;
        res[4*i + 3] = state[i] as u8;
    }
    res
}
