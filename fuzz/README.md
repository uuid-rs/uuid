# Fuzzing

This directory contains some fuzz testing infrastructure for `uuid`, using AFL++.

## Adding a new test case

1. Add a `fuzz/$TARGET_NAME/main.rs`, where `$TARGET_NAME` is a name for the test, like `parse`. See existing cases for the content to include in this file.
2. Add a `[[bin]]` to `fuzz/Cargo.toml` with the new target included.
3. Add an entry to the `fuzz/canary.sh` to run it.

## Running fuzz cases

You'll need to install `cargo.afl`, which can be done through Cargo:

```shell
cargo install -f argo-afl
```

Then, configure your system for fuzzing:

```shell
cargo afl config --build --force
cargo afl system-config
```

Now you can build the fuzz targets:

```shell
cargo afl build --manifest-path fuzz/Cargo.toml --features afl
```

And run one interactively:

```shell
cargo afl fuzz -i fuzz/$TARGET_NAME/in -o fuzz/target/$TARGET_NAME fuzz/target/debug/$TARGET_NAME
```

where `$TARGET_NAME` is the name of the fuzz test you want to run. This should show you an AFL TUI.

## Reproducing crashes

Crashes are saved into the `target` directory and are re-tested by the regular unit tests on each fuzz case. Just run:

```shell
cargo test --manifest-path fuzz/Cargo.toml
```

and any crashes will be re-tested.
