package io.github.honhimw.uuid;

/**
 * @author honhimW
 * @since 2025-10-09
 */

class InternalUuid {

    static native long[] v1(long counterPtr, long seconds, int nanos, byte[] nodeId);

    static native long[] v3(long m, long l, byte[] name);

    static native long[] v4();

    static native long[] v5(long m, long l, byte[] name);

    static native long[] v6(long counterPtr, long seconds, int nanos, byte[] nodeId);

    static native long[] v7(long counterPtr, long seconds, int nanos);

    static native long[] v8(byte[] bytes);

    /// Shared timestamp context

    static native long[] nowV1(byte[] nodeId);

    static native long[] nowV6(byte[] nodeId);

    static native long[] nowV7();

}
