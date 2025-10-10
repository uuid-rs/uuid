package io.github.honhimw.uuid;

import java.security.SecureRandom;
import java.util.UUID;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public interface Uuid {

    SecureRandom SECURE_RANDOM = new SecureRandom();

    UUID random();

}
