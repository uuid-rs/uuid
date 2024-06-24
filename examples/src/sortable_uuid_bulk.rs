//! Generating a sortable UUID.
//!
//! If you enable the `v7` feature you can generate sortable UUIDs.
//! This example avoids the synchronization cost of `Uuid::now_v7()`
//! when generating UUIDs in bulk.

fn main() {
    use uuid::{ContextV7, Timestamp, Uuid};

    let ctxt = ContextV7::new();

    for _ in 0..10 {
        println!("{}", Uuid::new_v7(Timestamp::now(&ctxt)));
    }
}
