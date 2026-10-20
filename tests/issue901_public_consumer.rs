#![cfg(all(
    feature = "v1",
    feature = "v3",
    feature = "v4",
    feature = "v5",
    feature = "v6",
    feature = "v7",
    feature = "v8",
    feature = "std",
    feature = "rng"
))]

use uuid::{ContextV1, NoContext, NonNilUuid, Timestamp, Uuid};

const CONST_V8: NonNilUuid = NonNilUuid::new_v8([0; 16]);

fn all_constructors() -> [NonNilUuid; 10] {
    let node = [1, 2, 3, 4, 5, 6];
    let context = ContextV1::new(42);
    let timestamp = Timestamp::from_unix(&context, 1_497_624_119, 1234);
    let timestamp_v7 = Timestamp::from_unix(NoContext, 1_497_624_119, 1234);

    [
        NonNilUuid::new_v1(timestamp, &node),
        NonNilUuid::now_v1(&node),
        NonNilUuid::new_v3(&Uuid::NAMESPACE_DNS, b"consumer"),
        NonNilUuid::new_v4(),
        NonNilUuid::new_v5(&Uuid::NAMESPACE_DNS, b"consumer"),
        NonNilUuid::new_v6(timestamp, &node),
        NonNilUuid::now_v6(&node),
        NonNilUuid::new_v7(timestamp_v7),
        NonNilUuid::now_v7(),
        CONST_V8,
    ]
}

#[test]
fn every_constructor_returns_non_nil() {
    for uuid in all_constructors() {
        assert!(!uuid.get().is_nil());
    }
}
