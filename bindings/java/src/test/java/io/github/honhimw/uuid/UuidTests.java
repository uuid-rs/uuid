package io.github.honhimw.uuid;

import org.junit.jupiter.api.Test;

import java.time.Instant;
import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class UuidTests extends BaseTest {

    @Test
    void v1() {
        UUID uuid = UUIDs.V1.random();
        assertUuid(uuid);
        uuid = UUIDs.V1.of(Instant.now(), MacAddress.tryGetFirst());
        assertUuid(uuid);
    }

    @Test
    void v3() {
        UUID uuid = UUIDs.V3.random();
        assertUuid(uuid);
    }

    @Test
    void v4() {
        UUID uuid = UUIDs.V4.random();
        assertUuid(uuid);
    }

    @Test
    void v5() {
        UUID uuid = UUIDs.V5.random();
        assertUuid(uuid);
    }

    @Test
    void v6() {
        UUID uuid = UUIDs.V6.random();
        assertUuid(uuid);
        uuid = UUIDs.V6.of(Instant.now(), MacAddress.tryGetFirst());
        assertUuid(uuid);
    }

    @Test
    void v7() {
        UUID uuid = UUIDs.V7.random();
        assertUuid(uuid);
        uuid = UUIDs.V7.of(Instant.now());
        assertUuid(uuid);
    }

    @Test
    void v8() {
        UUID uuid = UUIDs.V8.random();
        assertUuid(uuid);
        byte[] bytes = new byte[16];
        Uuid.SECURE_RANDOM.nextBytes(bytes);
        uuid = UUIDs.V8.of(bytes);
        assertUuid(uuid);
        uuid = UUIDs.V8.of(Uuid.SECURE_RANDOM.nextLong(), Uuid.SECURE_RANDOM.nextLong());
        assertUuid(uuid);
    }

}
