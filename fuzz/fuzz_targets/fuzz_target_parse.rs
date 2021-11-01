#![no_main]
use libfuzzer_sys::fuzz_target;

use std::str;
use uuid::Uuid;

fuzz_target!(|data: &[u8]| {
    if let Ok(uuid) = str::from_utf8(data) {
        // Ensure the parser doesn't panic
        let _ = Uuid::parse_str(uuid);
    }
});
