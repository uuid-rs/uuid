extern crate rustc_version;

fn main() {
    match rustc_version::version_meta() {
        Ok(meta) => {
            if let rustc_version::Channel::Nightly = meta.channel {
                println!("cargo:rustc-cfg=CARGO_FEATURE_NIGHTLY")
            }
        },
        Err(err) => {
            println!("{}", err)
        },
    }
}
