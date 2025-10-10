package io.github.honhimw.uuid;

import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class UUIDs {

    public static final V1 V1;
    public static final V3 V3;
    public static final V4 V4;
    public static final V5 V5;
    public static final V6 V6;
    public static final V7 V7;
    public static final V8 V8;

    static {
        LoadHelper.loadUuid();
        V1 = new V1();
        V3 = new V3();
        V4 = new V4();
        V5 = new V5();
        V6 = new V6();
        V7 = new V7();
        V8 = new V8();
    }

    private UUIDs() {
    }

    public static UUID from(long[] pair) {
        if (pair.length != 2) {
            throw new IllegalArgumentException("invalid uuid pair");
        }
        return new UUID(pair[0], pair[1]);
    }

    /// Low-Level API, one-to-one binding for rust API

    public static UUID v1(Context context, long seconds, int nanos, byte[] nodeId) {
        long[] longs = InternalUuid.v1(context.ptr(), seconds, nanos, nodeId);
        return from(longs);
    }

    public static UUID v3(UUID namespace, byte[] name) {
        long[] longs = InternalUuid.v3(namespace.getMostSignificantBits(), namespace.getLeastSignificantBits(), name);
        return from(longs);
    }

    public static UUID v4() {
        long[] longs = InternalUuid.v4();
        return from(longs);
    }

    public static UUID v5(UUID namespace, byte[] name) {
        long[] longs = InternalUuid.v5(namespace.getMostSignificantBits(), namespace.getLeastSignificantBits(), name);
        return from(longs);
    }

    public static UUID v6(Context context, long seconds, int nanos, byte[] nodeId) {
        long[] longs = InternalUuid.v6(context.ptr(), seconds, nanos, nodeId);
        return from(longs);
    }

    public static UUID v7(Context context, long seconds, int nanos) {
        long[] longs = InternalUuid.v7(context.ptr(), seconds, nanos);
        return from(longs);
    }

    public static UUID v8(byte[] bytes) {
        long[] longs = InternalUuid.v8(bytes);
        return from(longs);
    }

    public static UUID nowV1(byte[] nodeId) {
        long[] longs = InternalUuid.nowV1(nodeId);
        return from(longs);
    }

    public static UUID nowV6(byte[] nodeId) {
        long[] longs = InternalUuid.nowV6(nodeId);
        return from(longs);
    }

    public static UUID v7() {
        long[] longs = InternalUuid.nowV7();
        return from(longs);
    }

}
