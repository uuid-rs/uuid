// Gungraun benchmarks require the `gungraun-runner` binary and Valgrind, which
// aren't available on wasm32 targets, so the harness is compiled out there.
#[cfg(not(target_arch = "wasm32"))]
mod bench {
    use std::hint::black_box;

    use gungraun::prelude::*;
    use uuid::Uuid;

    #[library_benchmark]
    fn parse_nil() -> Result<Uuid, uuid::Error> {
        black_box(Uuid::parse_str(black_box("00000000000000000000000000000000")))
    }

    #[library_benchmark]
    fn parse_nil_hyphenated() -> Result<Uuid, uuid::Error> {
        black_box(Uuid::parse_str(black_box(
            "00000000-0000-0000-0000-000000000000",
        )))
    }

    #[library_benchmark]
    fn parse_random() -> Result<Uuid, uuid::Error> {
        black_box(Uuid::parse_str(black_box("67e5504410b1426f9247bb680e5fe0c8")))
    }

    #[library_benchmark]
    fn parse_random_hyphenated() -> Result<Uuid, uuid::Error> {
        black_box(Uuid::parse_str(black_box(
            "67e55044-10b1-426f-9247-bb680e5fe0c8",
        )))
    }

    #[library_benchmark]
    fn parse_urn() -> Result<Uuid, uuid::Error> {
        black_box(Uuid::parse_str(black_box(
            "urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8",
        )))
    }

    #[library_benchmark]
    fn parse_invalid_len() -> Result<Uuid, uuid::Error> {
        black_box(Uuid::parse_str(black_box("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4")))
    }

    #[library_benchmark]
    fn parse_invalid_character() -> Result<Uuid, uuid::Error> {
        black_box(Uuid::parse_str(black_box(
            "F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4",
        )))
    }

    #[library_benchmark]
    fn parse_invalid_group_len() -> Result<Uuid, uuid::Error> {
        black_box(Uuid::parse_str(black_box("01020304-1112-2122-3132-41424344")))
    }

    #[library_benchmark]
    fn parse_invalid_groups() -> Result<Uuid, uuid::Error> {
        black_box(Uuid::parse_str(black_box(
            "F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4",
        )))
    }

    library_benchmark_group!(
        name = parse_str,
        benchmarks = [
            parse_nil,
            parse_nil_hyphenated,
            parse_random,
            parse_random_hyphenated,
            parse_urn,
            parse_invalid_len,
            parse_invalid_character,
            parse_invalid_group_len,
            parse_invalid_groups
        ]
    );
}

#[cfg(not(target_arch = "wasm32"))]
use bench::parse_str;

#[cfg(not(target_arch = "wasm32"))]
gungraun::main!(library_benchmark_groups = parse_str);

#[cfg(target_arch = "wasm32")]
fn main() {}
