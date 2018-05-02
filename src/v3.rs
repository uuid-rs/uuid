use md5;
use prelude::*;

impl Uuid {
    /// Creates a [`Uuid`] using a name from a namespace, based on the MD5
    /// hash.
    ///
    /// A number of namespaces are available as constants in this crate:
    ///
    /// * [`NAMESPACE_DNS`]
    /// * [`NAMESPACE_OID`]
    /// * [`NAMESPACE_URL`]
    /// * [`NAMESPACE_X500`]
    ///
    /// Note that usage of this method requires the `v3` feature of this crate
    /// to be enabled.
    ///
    /// [`NAMESPACE_DNS`]: ../ns/const.NAMESPACE_DNS.html
    /// [`NAMESPACE_OID`]: ../ns/const.NAMESPACE_OID.html
    /// [`NAMESPACE_URL`]: ../ns/const.NAMESPACE_URL.html
    /// [`NAMESPACE_X500`]: ../ns/const.NAMESPACE_X500.html
    /// [`Uuid`]: ../struct.Uuid.html
    pub fn new_v3(namespace: &Uuid, name: &str) -> Self {
        let mut context = md5::Context::new();

        context.consume(namespace.as_bytes());
        context.consume(name.as_bytes());

        let mut uuid = Uuid {
            bytes: context.compute().into(),
        };

        uuid.set_variant(UuidVariant::RFC4122);
        uuid.set_version(UuidVersion::Md5);
        uuid
    }
}

#[cfg(test)]
mod tests {

    static FIXTURE: &'static [(&'static Uuid, &'static str, &'static str)] = {
        use ns::*;

        &[
            (
                &NAMESPACE_DNS,
                "example.org",
                "04738bdf-b25a-3829-a801-b21a1d25095b",
            ),
            (
                &NAMESPACE_DNS,
                "rust-lang.org",
                "c6db027c-615c-3b4d-959e-1a917747ca5a",
            ),
            (
                &NAMESPACE_DNS,
                "42",
                "5aab6e0c-b7d3-379c-92e3-2bfbb5572511",
            ),
            (
                &NAMESPACE_DNS,
                "lorem ipsum",
                "4f8772e9-b59c-3cc9-91a9-5c823df27281",
            ),
            (
                &NAMESPACE_URL,
                "example.org",
                "39682ca1-9168-3da2-a1bb-f4dbcde99bf9",
            ),
            (
                &NAMESPACE_URL,
                "rust-lang.org",
                "7ed45aaf-e75b-3130-8e33-ee4d9253b19f",
            ),
            (
                &NAMESPACE_URL,
                "42",
                "08998a0c-fcf4-34a9-b444-f2bfc15731dc",
            ),
            (
                &NAMESPACE_URL,
                "lorem ipsum",
                "e55ad2e6-fb89-34e8-b012-c5dde3cd67f0",
            ),
            (
                &NAMESPACE_OID,
                "example.org",
                "f14eec63-2812-3110-ad06-1625e5a4a5b2",
            ),
            (
                &NAMESPACE_OID,
                "rust-lang.org",
                "6506a0ec-4d79-3e18-8c2b-f2b6b34f2b6d",
            ),
            (
                &NAMESPACE_OID,
                "42",
                "ce6925a5-2cd7-327b-ab1c-4b375ac044e4",
            ),
            (
                &NAMESPACE_OID,
                "lorem ipsum",
                "5dd8654f-76ba-3d47-bc2e-4d6d3a78cb09",
            ),
            (
                &NAMESPACE_X500,
                "example.org",
                "64606f3f-bd63-363e-b946-fca13611b6f7",
            ),
            (
                &NAMESPACE_X500,
                "rust-lang.org",
                "bcee7a9c-52f1-30c6-a3cc-8c72ba634990",
            ),
            (
                &NAMESPACE_X500,
                "42",
                "c1073fa2-d4a6-3104-b21d-7a6bdcf39a23",
            ),
            (
                &NAMESPACE_X500,
                "lorem ipsum",
                "02f09a3f-1624-3b1d-8409-44eff7708208",
            ),
        ]
    };

    #[test]
    fn test_new() {
        for &(ref ns, ref name, _) in FIXTURE_V3 {
            let uuid = Uuid::new_v3(*ns, *name);
            assert_eq!(uuid.get_version().unwrap(), UuidVersion::Md5);
            assert_eq!(
                uuid.get_variant().unwrap(),
                UuidVariant::RFC4122
            );
        }
    }

    #[test]
    fn test_to_hyphenated_string() {
        for &(ref ns, ref name, ref expected) in FIXTURE_V3 {
            let uuid = Uuid::new_v3(*ns, *name);
            assert_eq!(uuid.hyphenated().to_string(), *expected);
        }
    }
}