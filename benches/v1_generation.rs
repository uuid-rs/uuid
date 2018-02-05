#[macro_use]
extern crate criterion;

extern crate uuid;

use std::sync::atomic::{AtomicUsize, Ordering};
use criterion::Criterion;

use uuid::{Uuid, UuidV1Context, DefaultUuidV1Context};

struct OldContext {
    count: AtomicUsize,
}
impl UuidV1Context for OldContext {
    fn generate(&self, _current_seconds: u64, _current_nanoseconds: u32) -> u16 {
        (self.count.fetch_add(1, Ordering::SeqCst) & 0xffff) as u16
    }
}


fn generate_v1s(c: &mut Criterion) {
    let node = [0, 1, 2, 3, 4, 5];
    c.bench_function("mutex context", |b| {
        let context = DefaultUuidV1Context::new(0);
        b.iter(move || {
            Uuid::new_v1(&context, 0, 0, &node[..])
        })
    });


    c.bench_function("mutex context increasing time", |b| {
        let mut time: u64 = 0;
        let context = DefaultUuidV1Context::new(0);
        b.iter(move || {
            time += 1;
            Uuid::new_v1(&context, time, 0, &node[..])
        })
    });

    c.bench_function("old context", |b| {
        let context = OldContext { count: AtomicUsize::new(0) };
        b.iter(move || {
            Uuid::new_v1(&context, 0, 0, &node[..])
        })
    });

    c.bench_function("old context increasing time", |b| {
        let mut time: u64 = 0;
        let context = OldContext { count: AtomicUsize::new(0) };
        b.iter(move || {
            time += 1;
            Uuid::new_v1(&context, time, 0, &node[..])
        })
    });

}
criterion_group!(benches, generate_v1s);
criterion_main!(benches);
