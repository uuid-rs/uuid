package io.github.honhimw.uuid;

import org.jspecify.annotations.Nullable;

import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.util.concurrent.atomic.AtomicBoolean;

/**
 * @author honhimW
 * @since 2025-10-09
 */

public class LoadHelper {

    private static final AtomicBoolean LOADED = new AtomicBoolean(false);

    public static void loadUuid() {
        if (LOADED.compareAndSet(false, true)) {
            load("uuid_java");
        }
    }

    public static void load(String name) {
        try {
            Platform platform = Platform.getPlatform(System.getProperty("os.name"));
            Arch arch = Arch.getArch(System.getProperty("os.arch"));
            if (platform == null || arch == null) {
                throw new RuntimeException("Unsupported OS: " + System.getProperty("os.name") + ", ARCH: " + System.getProperty("os.arch"));
            }
            Target target = Target.getTarget(platform, arch);
            if (target == null) {
                throw new RuntimeException("Unsupported OS: " + System.getProperty("os.name") + ", TARGET: " + System.getProperty("os.arch"));
            }
            String libName = "/" + name + "-" + target.target + platform.suffix;
            InputStream in = LoadHelper.class.getResourceAsStream(libName);
            if (in == null) {
                throw new RuntimeException("Library not found: " + libName);
            }
            File tempFile = File.createTempFile("rs_ffi_", platform.suffix);
            tempFile.deleteOnExit();
            byte[] buf = new byte[8192];
            FileOutputStream fileOutputStream = new FileOutputStream(tempFile);
            int bytesRead;
            while ((bytesRead = in.read(buf)) != -1) {
                fileOutputStream.write(buf, 0, bytesRead);
            }
            fileOutputStream.close();
            in.close();
            System.load(tempFile.getAbsolutePath());
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public enum Platform {
        WINDOWS("windows", ".dll"),
        MACOS("macos", ".dylib"),
        LINUX("linux", ".so");

        public final String name;
        public final String suffix;

        Platform(String name, String suffix) {
            this.name = name;
            this.suffix = suffix;
        }

        @Nullable
        public static Platform getPlatform(String os) {
            String osName = os.toLowerCase();
            for (Platform value : values()) {
                return osName.contains(value.name) ? value : null;
            }
            return null;
        }
    }

    public enum Arch {
        X86("x86"),
        AMD64("amd64", "x86_64"),
        AARCH64("aarch64"),
        ;

        private final String[] names;

        Arch(String... names) {
            this.names = names;
        }

        @Nullable
        public static Arch getArch(String arch) {
            for (Arch value : values()) {
                for (String name : value.names) {
                    if (name.equalsIgnoreCase(arch)) {
                        return value;
                    }
                }
            }
            return null;
        }

    }

    public enum Target {
        LINUX_X86_64("x86_64-unknown-linux-gnu"),
        LINUX_AARCH64("aarch64-unknown-linux-gnu"),
        WINDOWS_X86_64("x86_64-pc-windows-msvc"),
        WINDOWS_AARCH64("aarch64-pc-windows-msvc"),
        MACOS_AARCH64("aarch64-apple-darwin"),
        MACOS_X86_64("x86_64-apple-darwin"),
        ;

        private final String target;

        Target(String target) {
            this.target = target;
        }

        @Nullable
        public static Target getTarget(Platform platform, Arch arch) {
            switch (platform) {
                case LINUX -> {
                    switch (arch) {
                        case AMD64 -> {
                            return LINUX_X86_64;
                        }
                        case AARCH64 -> {
                            return LINUX_AARCH64;
                        }
                    }
                }
                case WINDOWS -> {
                    switch (arch) {
                        case AMD64 -> {
                            return WINDOWS_X86_64;
                        }
                        case AARCH64 -> {
                            return WINDOWS_AARCH64;
                        }
                    }
                }
                case MACOS -> {
                    switch (arch) {
                        case AARCH64 -> {
                            return MACOS_AARCH64;
                        }
                        case AMD64 -> {
                            return MACOS_X86_64;
                        }
                    }
                }
            }
            return null;
        }
    }

}
