use uuid::{uuid, Uuid};

const _: Uuid = uuid!("");
const _: Uuid = uuid!("!");
const _: Uuid = uuid!("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E45");
const _: Uuid = uuid!("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4");
const _: Uuid = uuid!("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4");
const _: Uuid = uuid!("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4");
const _: Uuid = uuid!("F9168C5E-CEB2-4faa");
const _: Uuid = uuid!("F9168C5E-CEB2-4faaXB6BFF329BF39FA1E4");
const _: Uuid = uuid!("F9168C5E-CEB-24fa-eB6BFF32-BF39FA1E4");
const _: Uuid = uuid!("01020304-1112-2122-3132-41424344");
const _: Uuid = uuid!("67e5504410b1426f9247bb680e5fe0c88");
const _: Uuid = uuid!("67e5504410b1426f9247bb680e5fe0cg8");
const _: Uuid = uuid!("67e5504410b1426%9247bb680e5fe0c8");

// Test error reporting
const _: Uuid = uuid!("67e5504410b1426f9247bb680e5fe0c");
const _: Uuid = uuid!("67e550X410b1426f9247bb680e5fe0cd");
const _: Uuid = uuid!("67e550-4105b1426f9247bb680e5fe0c");
const _: Uuid = uuid!("F9168C5E-CEB2-4faa-B6BF1-02BF39FA1E4");


const _: Uuid = uuid!("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4");
const _: Uuid = uuid!("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4");
const _: Uuid = uuid!("01020304-1112-2122-3132-41424344");
const _: Uuid = uuid!("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4");

fn main() {}