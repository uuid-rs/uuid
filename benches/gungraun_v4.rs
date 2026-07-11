// Gungraun benchmarks require the `gungraun-runner` binary and Valgrind, which
// aren't available on wasm32 targets, so the harness is compiled out there.
#[cfg(all(not(target_arch = "wasm32"), feature = "v4"))]
mod bench {
    use std::hint::black_box;

    use gungraun::prelude::*;
    use uuid::Uuid;

    #[library_benchmark]
    fn new_v4() -> Uuid {
        black_box(Uuid::new_v4())
    }

    library_benchmark_group!(
        name = v4,
        benchmarks = [new_v4]
    );
}

#[cfg(all(not(target_arch = "wasm32"), feature = "v4"))]
use bench::v4;

#[cfg(all(not(target_arch = "wasm32"), feature = "v4"))]
gungraun::main!(library_benchmark_groups = v4);

#[cfg(not(all(not(target_arch = "wasm32"), feature = "v4")))]
fn main() {}
