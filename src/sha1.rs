pub const K0: u32 = 0x5A827999u32;
pub const K1: u32 = 0x6ED9EBA1u32;
pub const K2: u32 = 0x8F1BBCDCu32;
pub const K3: u32 = 0xCA62C1D6u32;

pub const H: [u32; 5] = [
    0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0
];

const BLOCK_SIZE: usize = 64;

fn compress(state: &mut [u32; 5], block: &[u8]) {
    assert_eq!(block.len(), 64);
    let mut words = [0u32; 80];

    for i in 0..16 {
        let off = i * 4;
        words[i] = (block[off + 3] as u32) |
                   ((block[off + 2] as u32) << 8) |
                   ((block[off + 1] as u32) << 16) |
                   ((block[off] as u32) << 24);
    }

    fn ff(b: u32, c: u32, d: u32) -> u32 {
        d ^ (b & (c ^ d))
    }
    fn gg(b: u32, c: u32, d: u32) -> u32 {
        b ^ c ^ d
    }
    fn hh(b: u32, c: u32, d: u32) -> u32 {
        (b & c) | (d & (b | c))
    }
    fn ii(b: u32, c: u32, d: u32) -> u32 {
        b ^ c ^ d
    }

    for i in 16..80 {
        let n = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
        words[i] = n.rotate_left(1);
    }

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];

    for i in 0..80 {
        let (f, k) = match i {
            0...19 => (ff(b, c, d), K0),
            20...39 => (gg(b, c, d), K1),
            40...59 => (hh(b, c, d), K2),
            60...79 => (ii(b, c, d), K3),
            _ => (0, 0),
        };

        let tmp = a.rotate_left(5)
            .wrapping_add(f)
            .wrapping_add(e)
            .wrapping_add(k)
            .wrapping_add(words[i]);
        e = d;
        d = c;
        c = b.rotate_left(30);
        b = a;
        a = tmp;
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
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
