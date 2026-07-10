// Gungraun benchmarks require the `gungraun-runner` binary and Valgrind, which
// aren't available on wasm32 targets, so the harness is compiled out there.
#[cfg(not(target_arch = "wasm32"))]
mod bench {
    use std::hint::black_box;
    use std::io::Write;

    use gungraun::prelude::*;
    use uuid::Uuid;

    fn setup_uuid() -> Uuid {
        Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4").unwrap()
    }

    #[library_benchmark]
    #[bench::default(setup_uuid())]
    fn hyphenated(uuid: Uuid) -> [u8; 36] {
        let mut buffer = [0_u8; 36];
        write!(&mut buffer as &mut [_], "{:x}", uuid.hyphenated()).unwrap();
        black_box(buffer)
    }

    #[library_benchmark]
    #[bench::default(setup_uuid())]
    fn simple(uuid: Uuid) -> [u8; 32] {
        let mut buffer = [0_u8; 32];
        write!(&mut buffer as &mut [_], "{:x}", uuid.simple()).unwrap();
        black_box(buffer)
    }

    #[library_benchmark]
    #[bench::default(setup_uuid())]
    fn urn(uuid: Uuid) -> [u8; 36 + 9] {
        let mut buffer = [0_u8; 36 + 9];
        write!(&mut buffer as &mut [_], "{:x}", uuid.urn()).unwrap();
        black_box(buffer)
    }

    #[library_benchmark]
    #[bench::default(setup_uuid())]
    fn encode_hyphen(uuid: Uuid) -> [u8; 36] {
        let mut buffer = [0_u8; 36];
        uuid.hyphenated().encode_lower(&mut buffer);
        black_box(buffer)
    }

    #[library_benchmark]
    #[bench::default(setup_uuid())]
    fn encode_simple(uuid: Uuid) -> [u8; 32] {
        let mut buffer = [0_u8; 32];
        uuid.simple().encode_lower(&mut buffer);
        black_box(buffer)
    }

    #[library_benchmark]
    #[bench::default(setup_uuid())]
    fn encode_urn(uuid: Uuid) -> [u8; 36 + 9] {
        let mut buffer = [0_u8; 36 + 9];
        uuid.urn().encode_lower(&mut buffer);
        black_box(buffer)
    }

    library_benchmark_group!(
        name = format_str,
        benchmarks = [
            hyphenated,
            simple,
            urn,
            encode_hyphen,
            encode_simple,
            encode_urn
        ]
    );
}

#[cfg(not(target_arch = "wasm32"))]
use bench::format_str;

#[cfg(not(target_arch = "wasm32"))]
gungraun::main!(library_benchmark_groups = format_str);

#[cfg(target_arch = "wasm32")]
fn main() {}
