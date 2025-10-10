package io.github.honhimw.uuid;

import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class V4 implements Uuid {

    @Override
    public UUID random() {
        long[] longs = InternalUuid.v4();
        return UUIDs.from(longs);
    }

}
