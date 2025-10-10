package io.github.honhimw.uuid.bench;

import org.junit.jupiter.api.Test;
import org.openjdk.jmh.results.format.ResultFormatType;
import org.openjdk.jmh.runner.Runner;
import org.openjdk.jmh.runner.RunnerException;
import org.openjdk.jmh.runner.options.Options;
import org.openjdk.jmh.runner.options.OptionsBuilder;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class BenchmarkRunner {

    @Test
    void benchmark() throws RunnerException {
        Options options = new OptionsBuilder()
            .include(Self.class.getSimpleName())
            .include(AlternativeJdk.class.getSimpleName())
            .include(Jdk.class.getSimpleName())
            .threads(8)
            .forks(1)
            .resultFormat(ResultFormatType.LATEX)
            .build();
        new Runner(options).run();
    }

}
