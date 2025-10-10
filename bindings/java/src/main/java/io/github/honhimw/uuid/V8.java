package io.github.honhimw.uuid;

import java.nio.ByteBuffer;
import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class V8 implements Uuid {

    public UUID of(long m, long l) {
        byte[] array = ByteBuffer.allocate(Long.BYTES * 2).putLong(m).putLong(l).array();
        return of(array);
    }

    public UUID of(byte[] bytes) {
        if (bytes.length != 16) {
            throw new IllegalArgumentException("bytes.length != 16");
        }
        long[] longs = InternalUuid.v8(bytes);
        return UUIDs.from(longs);
    }

    @Override
    public UUID random() {
        byte[] bytes = new byte[16];
        SECURE_RANDOM.nextBytes(bytes);
        return of(bytes);
    }

}
