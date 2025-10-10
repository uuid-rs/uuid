package io.github.honhimw.uuid;

import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public abstract class BaseTest {

    static {
        LoadHelper.loadUuid();
    }

    protected void assertUuid(UUID uuid) {
        assert uuid.getMostSignificantBits() != 0;
        assert uuid.getLeastSignificantBits() != 0;
    }

}
