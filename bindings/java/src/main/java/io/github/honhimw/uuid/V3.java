package io.github.honhimw.uuid;

import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class V3 implements Uuid {

    @Override
    public UUID random() {
        UUID uuid = UUIDs.V4.random();
        byte[] bytes = new byte[SECURE_RANDOM.nextInt(4, 64)];
        SECURE_RANDOM.nextBytes(bytes);
        long[] longs = InternalUuid.v3(uuid.getMostSignificantBits(), uuid.getLeastSignificantBits(), bytes);
        return UUIDs.from(longs);
    }

}
