use std::{fs, io::Read, panic};

pub fn main(fuzz_fn: impl Fn(&[u8]) + panic::RefUnwindSafe) {
    #[cfg(feature = "afl")]
    {
        afl::fuzz!(|input: &[u8]| { fuzz_fn(input) });
    }
    #[cfg(not(feature = "afl"))]
    {
        let _ = fuzz_fn;

        panic!("must be run with the `afl` feature")
    }
}

pub fn initial_cases(fuzz_target: &str, fuzz_fn: impl Fn(&[u8]) + panic::RefUnwindSafe) {
    let dir = format!("{}/{fuzz_target}/in", env!("CARGO_MANIFEST_DIR"),);

    println!("running cases in {dir}");

    let mut any = false;
    for input in fs::read_dir(&dir).expect("failed to read inputs directory") {
        any = true;

        let input = input.expect("invalid file").path();

        println!("input: {:?}", input);

        let mut f = fs::File::open(input).expect("failed to open");
        let mut input = Vec::new();
        f.read_to_end(&mut input).expect("failed to read file");

        fuzz_fn(&input);
    }

    assert!(any, "no test cases were executed");
}

pub fn repro_cases(fuzz_target: &str, fuzz_fn: impl Fn(&[u8]) + panic::RefUnwindSafe) {
    let dir = format!(
        "{}/target/{fuzz_target}/default",
        env!("CARGO_MANIFEST_DIR"),
    );

    println!("running cases in {dir}");

    if let Ok(crashes) = fs::read_dir(format!("{dir}/crashes")) {
        let mut failed = false;
        for crash in crashes {
            let crash = crash.expect("invalid file").path();

            if let Some("README.txt") = crash.file_name().and_then(|name| name.to_str()) {
                continue;
            }

            println!("\n-----\nrepro: {crash:?}");

            let mut f = fs::File::open(crash).expect("failed to open");
            let mut crash = Vec::new();
            f.read_to_end(&mut crash).expect("failed to read file");

            println!("repro: {crash:?}");
            println!("repro: {:?}", String::from_utf8_lossy(&crash));

            if let Err(_) = panic::catch_unwind(|| fuzz_fn(&crash)) {
                failed = true;
            }

            println!("-----");
        }

        if failed {
            panic!("some cases failed (see output above for details)");
        }
    } else {
        #[cfg(feature = "force")]
        {
            assert!(fs::exists(&dir).expect("failed to get file info"), "the {fuzz_target} target didn't execute; this probably means the fuzzing harness is broken");
        }
    }
}
