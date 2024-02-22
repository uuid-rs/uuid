#[cfg(any(feature = "v4", feature = "v7"))]
pub(crate) fn bytes() -> [u8; 16] {
    #[cfg(not(feature = "fast-rng"))]
    {
        let mut bytes = [0u8; 16];

        getrandom::getrandom(&mut bytes).unwrap_or_else(|err| {
            // NB: getrandom::Error has no source; this is adequate display
            panic!("could not retrieve random bytes for uuid: {}", err)
        });

        bytes
    }

    #[cfg(feature = "fast-rng")]
    {
        rand::random()
    }
}

#[cfg(any(feature = "v1", feature = "v6"))]
pub(crate) fn u16() -> u16 {
    #[cfg(not(feature = "fast-rng"))]
    {
        let mut bytes = [0u8; 2];

        getrandom::getrandom(&mut bytes).unwrap_or_else(|err| {
            // NB: getrandom::Error has no source; this is adequate display
            panic!("could not retrieve random bytes for uuid: {}", err)
        });

        ((bytes[0] as u16) << 8) | (bytes[1] as u16)
    }

    #[cfg(feature = "fast-rng")]
    {
        rand::random()
    }
}

#[cfg(feature = "v1_auto")]
pub(crate) fn fill_random_bytes(buf: &mut [u8]) {
    #[cfg(not(feature = "fast-rng"))]
    {
        getrandom::getrandom(buf).unwrap_or_else(|err| {
            // NB: getrandom::Error has no source; this is adequate display
            panic!("could not retrieve random bytes for node id: {}", err)
        });
    }

    #[cfg(feature = "fast-rng")]
    {
        use rand::RngCore;
        rand::thread_rng().fill_bytes(buf);
    }
}