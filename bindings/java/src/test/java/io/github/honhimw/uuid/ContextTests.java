package io.github.honhimw.uuid;

import org.junit.jupiter.api.Test;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class ContextTests extends BaseTest {

    @Test
    void construct() {
        Context context = new Context();
        System.out.println(context.ptr());
        context.init((short) 1);
        System.out.println(context.ptr());
        context.free();
        System.out.println(context.ptr());

    }

}
