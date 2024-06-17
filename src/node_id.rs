use std::sync::OnceLock;

static NODE_ID: OnceLock<[u8; 6]> = OnceLock::new();

pub(crate) fn get_or_make_node_id() -> &'static [u8; 6] {
    NODE_ID.get_or_init(|| match mac_address::get_mac_address() {
        Ok(Some(mac)) => mac.bytes(),
        _ => make_random_node_id(),
    })
}

fn make_random_node_id() -> [u8; 6] {
    let mut rand_bytes = [0u8; 6];

    crate::rng::fill_random_bytes(&mut rand_bytes);

    // set multicast bit
    rand_bytes[0] = rand_bytes[0] | 0x01;
    rand_bytes
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_multicast_bit_set() {
        // non deterministic test
        let node1 = super::make_random_node_id();
        assert_eq!(node1[0] & 0x01, 1);
    }
}
