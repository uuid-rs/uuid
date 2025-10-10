package io.github.honhimw.uuid.bench;

import io.github.honhimw.uuid.AlternativeJDKUUID;

import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class AlternativeJdk extends AbstractBenchmarks {

    private static final AlternativeJDKUUID UUID = new AlternativeJDKUUID();

    @Override
    public void get() throws Exception {
        UUID uuid = UUID.randomUUID();
        uuid.toString();
    }
}
