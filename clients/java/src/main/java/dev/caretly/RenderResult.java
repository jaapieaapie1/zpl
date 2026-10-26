package dev.caretly;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Objects;
import java.util.Optional;
import java.util.OptionalInt;

/**
 * A rendered PDF or PNG, with the metadata from the response headers. Immutable.
 */
public final class RenderResult {
    private final byte[] bytes;
    private final String contentType;
    private final int pageCount;
    private final List<Integer> filePageCounts;
    private final Integer labelsBilled;
    private final RateLimit rateLimit;
    private final String serverTiming;

    RenderResult(byte[] bytes, String contentType, int pageCount, List<Integer> filePageCounts,
            Integer labelsBilled, RateLimit rateLimit, String serverTiming) {
        this.bytes = Objects.requireNonNull(bytes, "bytes");
        this.contentType = Objects.requireNonNull(contentType, "contentType");
        this.pageCount = pageCount;
        this.filePageCounts = List.copyOf(filePageCounts);
        this.labelsBilled = labelsBilled;
        this.rateLimit = rateLimit;
        this.serverTiming = serverTiming;
    }

    /**
     * Returns a copy of the file bytes.
     *
     * @return the PDF or PNG bytes
     */
    public byte[] bytes() {
        return bytes.clone();
    }

    /**
     * Returns the size of the file in bytes.
     *
     * @return the byte count
     */
    public int size() {
        return bytes.length;
    }

    /**
     * Returns the media type of the file.
     *
     * @return {@code "application/pdf"} or {@code "image/png"}
     */
    public String contentType() {
        return contentType;
    }

    /**
     * Returns the total number of pages across the input, from {@code X-Page-Count}.
     *
     * @return the page count
     */
    public int pageCount() {
        return pageCount;
    }

    /**
     * Returns the pages per file in upload order, from {@code X-File-Page-Counts}.
     *
     * @return an unmodifiable list; empty for a single-label request
     */
    public List<Integer> filePageCounts() {
        return filePageCounts;
    }

    /**
     * Returns the labels billed for this request, from {@code X-Labels-Billed}: every page for
     * a live key, 1 for a PNG, 0 for a test key.
     *
     * @return the labels billed; empty when the server runs without API keys
     */
    public OptionalInt labelsBilled() {
        return labelsBilled == null ? OptionalInt.empty() : OptionalInt.of(labelsBilled);
    }

    /**
     * Returns the rate limit of the API key, from {@code RateLimit-Limit} and
     * {@code RateLimit-Remaining}.
     *
     * @return the rate limit; empty when the server sent no rate limit headers
     */
    public Optional<RateLimit> rateLimit() {
        return Optional.ofNullable(rateLimit);
    }

    /**
     * Returns the raw {@code Server-Timing} header, sent when timings were requested.
     *
     * @return the header, for example {@code "read;dur=0.1, parse;dur=0.4"}
     */
    public Optional<String> serverTiming() {
        return Optional.ofNullable(serverTiming);
    }

    /**
     * Writes the file to {@code path}, replacing it if it exists.
     *
     * @param path the file to write
     * @return {@code path}
     * @throws UncheckedIOException if the file cannot be written
     */
    public Path saveTo(Path path) {
        Objects.requireNonNull(path, "path");
        try {
            return Files.write(path, bytes);
        } catch (IOException e) {
            throw new UncheckedIOException(e);
        }
    }

    @Override
    public String toString() {
        return "RenderResult{contentType=" + contentType + ", size=" + bytes.length
                + ", pageCount=" + pageCount + ", filePageCounts=" + filePageCounts
                + ", labelsBilled=" + labelsBilled + ", rateLimit=" + rateLimit
                + ", serverTiming=" + serverTiming + "}";
    }
}
