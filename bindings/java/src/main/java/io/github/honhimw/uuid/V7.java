package io.github.honhimw.uuid;

import java.time.Instant;
import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class V7 implements Uuid, AutoCloseable {

    private final Context context;

    public V7() {
        this.context = new Context();
        context.init();
    }

    public V7(Context context) {
        this.context = context;
    }

    public UUID of(Instant timestamp) {
        long[] longs = InternalUuid.v7(context.ptr(), timestamp.getEpochSecond(), timestamp.getNano());
        return UUIDs.from(longs);
    }

    @Override
    public UUID random() {
        return UUIDs.from(InternalUuid.nowV7());
    }

    @Override
    public void close() throws Exception {
        this.context.close();
    }
}
