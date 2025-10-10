package io.github.honhimw.uuid.bench;

import org.openjdk.jmh.annotations.*;

import java.util.concurrent.TimeUnit;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public abstract class AbstractBenchmarks {

    @Setup(Level.Trial)
    public void setup() {
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.MILLISECONDS)
    @Warmup(iterations = 5, time = 2, timeUnit = TimeUnit.SECONDS)
    @Measurement(iterations = 10, time = 100, timeUnit = TimeUnit.SECONDS)
    @Fork(1)
    public void get() throws Exception {
    }

}
