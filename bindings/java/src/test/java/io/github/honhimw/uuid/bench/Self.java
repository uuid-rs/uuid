package io.github.honhimw.uuid.bench;

import io.github.honhimw.uuid.UUIDs;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class Self extends AbstractBenchmarks {

    @Override
    public void get() throws Exception {
        UUIDs.V4.random();
    }
}
