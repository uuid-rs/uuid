// Gungraun benchmarks require the `gungraun-runner` binary and Valgrind, which
// aren't available on wasm32 targets, so the harness is compiled out there.
#[cfg(all(not(target_arch = "wasm32"), feature = "v7", feature = "std"))]
mod bench {
    use std::hint::black_box;
    use std::time::SystemTime;

    use gungraun::prelude::*;
    use uuid::{ContextV7, NoContext, Timestamp, Uuid};

    #[library_benchmark]
    fn now_v7() -> Uuid {
        black_box(Uuid::now_v7())
    }

    #[library_benchmark]
    fn new_v7_no_context() -> Uuid {
        black_box(Uuid::new_v7(Timestamp::now(NoContext)))
    }

    fn setup_context() -> ContextV7 {
        ContextV7::new()
    }

    #[library_benchmark]
    #[bench::default(setup_context())]
    fn new_v7_context(ctxt: ContextV7) -> Uuid {
        black_box(Uuid::new_v7(Timestamp::now(&ctxt)))
    }

    fn setup_context_additional_precision() -> ContextV7 {
        ContextV7::new().with_additional_precision()
    }

    #[library_benchmark]
    #[bench::default(setup_context_additional_precision())]
    fn new_v7_context_additional_precision(ctxt: ContextV7) -> Uuid {
        black_box(Uuid::new_v7(Timestamp::now(&ctxt)))
    }

    fn setup_raw() -> (u64, u32) {
        let now = SystemTime::UNIX_EPOCH.elapsed().unwrap();
        (now.as_secs(), now.subsec_nanos())
    }

    #[library_benchmark]
    #[bench::default(setup_raw())]
    fn v7_raw((secs, subsec_nanos): (u64, u32)) -> Uuid {
        let mut counter = 0;
        black_box(Uuid::new_v7(Timestamp::from_unix_time(
            secs,
            subsec_nanos,
            {
                counter += 1;
                counter
            },
            42,
        )))
    }

    library_benchmark_group!(
        name = v7,
        benchmarks = [
            now_v7,
            new_v7_no_context,
            new_v7_context,
            new_v7_context_additional_precision,
            v7_raw
        ]
    );
}

#[cfg(all(not(target_arch = "wasm32"), feature = "v7", feature = "std"))]
use bench::v7;

#[cfg(all(not(target_arch = "wasm32"), feature = "v7", feature = "std"))]
gungraun::main!(library_benchmark_groups = v7);

#[cfg(not(all(not(target_arch = "wasm32"), feature = "v7", feature = "std")))]
fn main() {}
