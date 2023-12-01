package com.plasmoverse.vad;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.AccessDeniedException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

public final class VadLibrary {

    private static boolean LOADED;

    /**
     * Extracts the native library to the temp dir and loads it.
     *
     * @throws IOException If an error occurs while extracting the native library.
     * @throws UnsatisfiedLinkError If the native library fail to load.
     */
    public static void load() throws IOException {
        if (LOADED) return;

        File temporaryDir = Files.createTempDirectory("vad-jni-rust").toFile();
        temporaryDir.deleteOnExit();

        String libraryName = getPlatformLibraryFileName("vad_jni_rust");
        String platformFolder = getPlatformFolderName();
        String nativeLibraryPath = String.format("/natives/%s/%s", platformFolder, libraryName);

        InputStream source = VadLibrary.class.getResourceAsStream(nativeLibraryPath);
        if (source == null) {
            throw new IOException("Couldn't find the native library: " + nativeLibraryPath);
        }

        Path destination = temporaryDir.toPath().resolve(libraryName);
        try {
            Files.copy(source, destination, StandardCopyOption.REPLACE_EXISTING);
        } catch (AccessDeniedException ignored) {
        }
        System.load(destination.toFile().getAbsolutePath());

        LOADED = true;
    }

    private static String getPlatformFolderName() {
        return String.format(
                "%s-%s",
                getPlatformName(),
                getPlatformArch()
        );
    }

    private static String getPlatformName() {
        String systemName = System.getProperty("os.name").toLowerCase();

        if (systemName.contains("nux") || systemName.contains("nix")) {
            return "linux";
        } else if (systemName.contains("mac")) {
            return "mac";
        } else if (systemName.contains("windows")) {
            return "win";
        } else {
            throw new IllegalStateException("System is not supported: " + systemName);
        }
    }

    private static String getPlatformArch() {
        String systemArch = System.getProperty("os.arch").toLowerCase();

        boolean is64bit = systemArch.contains("64");
        boolean isArm = systemArch.startsWith("aarch");

        if (isArm) {
            return "aarch64";
        } else if (is64bit) {
            return "x86_64";
        } else {
            return "x86";
        }
    }

    private static String getPlatformLibraryFileName(String library) {
        String systemName = System.getProperty("os.name").toLowerCase();

        if (systemName.contains("nux") || systemName.contains("nix")) {
            return "lib" + library + ".so";
        } else if (systemName.contains("mac")) {
            return "lib" + library + ".dylib";
        } else if (systemName.contains("windows")) {
            return library + ".dll";
        } else {
            throw new IllegalStateException("System is not supported: " + systemName);
        }
    }

    private VadLibrary() {
    }
}
