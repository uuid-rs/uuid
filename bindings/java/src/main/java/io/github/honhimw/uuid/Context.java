package io.github.honhimw.uuid;

import java.security.SecureRandom;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class Context implements AutoCloseable {

    private long contextPtr;

    public void init() {
        short s = 0;
        try {
            SecureRandom instanceStrong = SecureRandom.getInstanceStrong();
            int i = instanceStrong.nextInt(Short.MIN_VALUE, Short.MAX_VALUE);
            s = (short) i;
        } catch (Exception ignored) {
        }
        init(s);
    }

    public native void init(short counter);

    public native void free();

    protected long ptr() {
        return contextPtr;
    }

    @Override
    public void close() throws Exception {
        synchronized (this) {
            if (contextPtr != 0) {
                free();
            }
        }
    }
}
