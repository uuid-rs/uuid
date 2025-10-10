package io.github.honhimw.uuid;

import java.time.Instant;
import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class V6 implements Uuid, AutoCloseable {

    private final Context context;

    public V6() {
        this.context = new Context();
        context.init();
    }

    public V6(Context context) {
        this.context = context;
    }

    public UUID of(Instant timestamp, byte[] nodeId) {
        if (nodeId.length != 6) {
            throw new IllegalArgumentException("nodeId.length != 6");
        }
        long[] longs = InternalUuid.v6(context.ptr(), timestamp.getEpochSecond(), timestamp.getNano(), nodeId);
        return UUIDs.from(longs);
    }

    @Override
    public UUID random() {
        byte[] bytes = new byte[6];
        SECURE_RANDOM.nextBytes(bytes);
        return UUIDs.from(InternalUuid.nowV6(bytes));
    }

    @Override
    public void close() throws Exception {
        this.context.close();
    }
}
