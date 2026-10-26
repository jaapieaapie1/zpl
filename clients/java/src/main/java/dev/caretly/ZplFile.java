package dev.caretly;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Objects;

/**
 * A named ZPL file for {@link CaretlyClient#renderFiles(java.util.List, RenderOptions)}.
 *
 * <p>Each file is rendered with a fresh printer state, and pages follow upload order.
 */
public final class ZplFile {
    private final String name;
    private final byte[] content;

    private ZplFile(String name, byte[] content) {
        this.name = Objects.requireNonNull(name, "name");
        this.content = Objects.requireNonNull(content, "content");
        if (name.isEmpty()) {
            throw new IllegalArgumentException("name must not be empty");
        }
    }

    /**
     * Creates a file from ZPL text, encoded as UTF-8.
     *
     * @param name the file name sent with the upload, for example {@code "order-1.zpl"}
     * @param zpl the ZPL
     * @return the file
     */
    public static ZplFile of(String name, String zpl) {
        Objects.requireNonNull(zpl, "zpl");
        return new ZplFile(name, zpl.getBytes(StandardCharsets.UTF_8));
    }

    /**
     * Creates a file from raw ZPL bytes.
     *
     * @param name the file name sent with the upload
     * @param zpl the ZPL bytes; the array is copied
     * @return the file
     */
    public static ZplFile of(String name, byte[] zpl) {
        Objects.requireNonNull(zpl, "zpl");
        return new ZplFile(name, zpl.clone());
    }

    /**
     * Reads a file from disk. The upload is named after the path's file name.
     *
     * @param path the ZPL file
     * @return the file
     * @throws UncheckedIOException if the file cannot be read
     */
    public static ZplFile fromPath(Path path) {
        Objects.requireNonNull(path, "path");
        try {
            return new ZplFile(String.valueOf(path.getFileName()), Files.readAllBytes(path));
        } catch (IOException e) {
            throw new UncheckedIOException(e);
        }
    }

    /**
     * Returns the file name sent with the upload.
     *
     * @return the file name
     */
    public String name() {
        return name;
    }

    /**
     * Returns a copy of the ZPL bytes.
     *
     * @return the ZPL bytes
     */
    public byte[] content() {
        return content.clone();
    }

    byte[] contentUnsafe() {
        return content;
    }
}
