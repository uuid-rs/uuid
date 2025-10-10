package io.github.honhimw.uuid;

import java.net.NetworkInterface;
import java.net.SocketException;
import java.util.Enumeration;

/**
 * @author honhimW
 * @since 2025-10-10
 */

public class MacAddress {

    private static final byte[] EMPTY_ADDRESS = new byte[6];

    public static byte[] tryGetFirst() {
        try {
            Enumeration<NetworkInterface> netInterfaces = NetworkInterface.getNetworkInterfaces();
            while (netInterfaces.hasMoreElements()) {
                NetworkInterface ni = netInterfaces.nextElement();
                byte[] hardwareAddress = ni.getHardwareAddress();
                if (hardwareAddress != null) {
                    return hardwareAddress;
                }
            }
        } catch (SocketException ignored) {
        }
        return EMPTY_ADDRESS;
    }

}
