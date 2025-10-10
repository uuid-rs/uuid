package io.github.honhimw.uuid.bench;

import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class Jdk extends AbstractBenchmarks {

    @Override
    public void get() throws Exception {
        UUID uuid = UUID.randomUUID();
        uuid.toString();
    }
}
