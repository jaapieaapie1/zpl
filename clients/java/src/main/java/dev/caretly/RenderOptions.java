package dev.caretly;

import java.util.Objects;
import java.util.Optional;
import java.util.OptionalInt;

/**
 * Options for a render. Unset options are not sent, so the server defaults apply
 * (PDF, page 1, 8 dpmm).
 *
 * <pre>{@code
 * RenderOptions options = RenderOptions.builder()
 *     .format(Format.PNG)
 *     .page(2)
 *     .dpmm(12)
 *     .build();
 * }</pre>
 *
 * <p>Instances are immutable and thread safe.
 */
public final class RenderOptions {
    private static final RenderOptions DEFAULTS = new Builder().build();

    private final Format format;
    private final Integer page;
    private final Integer dpmm;
    private final boolean timings;
    private final String idempotencyKey;
    private final boolean compress;

    private RenderOptions(Builder b) {
        this.format = b.format;
        this.page = b.page;
        this.dpmm = b.dpmm;
        this.timings = b.timings;
        this.idempotencyKey = b.idempotencyKey;
        this.compress = b.compress;
    }

    /**
     * Returns options with nothing set, so every server default applies.
     *
     * @return the default options
     */
    public static RenderOptions defaults() {
        return DEFAULTS;
    }

    /**
     * Returns a new builder.
     *
     * @return a builder with nothing set
     */
    public static Builder builder() {
        return new Builder();
    }

    /**
     * Returns a builder that starts from these options.
     *
     * @return a builder with these options set
     */
    public Builder toBuilder() {
        Builder b = new Builder();
        b.format = format;
        b.page = page;
        b.dpmm = dpmm;
        b.timings = timings;
        b.idempotencyKey = idempotencyKey;
        b.compress = compress;
        return b;
    }

    /**
     * Returns the output format, if set.
     *
     * @return the format; empty means the server default (PDF)
     */
    public Optional<Format> format() {
        return Optional.ofNullable(format);
    }

    /**
     * Returns the page to render as PNG, if set.
     *
     * @return the 1-based page; empty means the server default (1)
     */
    public OptionalInt page() {
        return page == null ? OptionalInt.empty() : OptionalInt.of(page);
    }

    /**
     * Returns the printer resolution, if set.
     *
     * @return the dots per millimetre; empty means the server default (8)
     */
    public OptionalInt dpmm() {
        return dpmm == null ? OptionalInt.empty() : OptionalInt.of(dpmm);
    }

    /**
     * Returns whether server timings are requested.
     *
     * @return {@code true} to ask for a {@code Server-Timing} header
     */
    public boolean timings() {
        return timings;
    }

    /**
     * Returns the idempotency key, if set.
     *
     * @return the key sent as {@code Idempotency-Key}
     */
    public Optional<String> idempotencyKey() {
        return Optional.ofNullable(idempotencyKey);
    }

    /**
     * Returns whether the request body is gzip compressed.
     *
     * @return {@code true} to send the body with {@code Content-Encoding: gzip}
     */
    public boolean compress() {
        return compress;
    }

    @Override
    public String toString() {
        return "RenderOptions{format=" + format + ", page=" + page + ", dpmm=" + dpmm
                + ", timings=" + timings + ", idempotencyKey=" + idempotencyKey
                + ", compress=" + compress + "}";
    }

    /** Builds {@link RenderOptions}. Setters validate their argument and fail fast. */
    public static final class Builder {
        private Format format;
        private Integer page;
        private Integer dpmm;
        private boolean timings;
        private String idempotencyKey;
        private boolean compress;

        private Builder() {
        }

        /**
         * Sets the output format.
         *
         * @param format {@link Format#PDF} (every page) or {@link Format#PNG} (one page)
         * @return this builder
         */
        public Builder format(Format format) {
            this.format = Objects.requireNonNull(format, "format");
            return this;
        }

        /**
         * Sets the page to render as PNG. Pages are counted across all files of a
         * multipart request. A PDF always holds every page.
         *
         * @param page the 1-based page
         * @return this builder
         * @throws IllegalArgumentException if {@code page} is less than 1
         */
        public Builder page(int page) {
            if (page < 1) {
                throw new IllegalArgumentException("page must be at least 1, got " + page);
            }
            this.page = page;
            return this;
        }

        /**
         * Sets the printer resolution in dots per millimetre: 6 (152 dpi), 8 (203 dpi),
         * 12 (300 dpi) or 24 (600 dpi).
         *
         * @param dpmm the dots per millimetre
         * @return this builder
         * @throws IllegalArgumentException if {@code dpmm} is not 6, 8, 12 or 24
         */
        public Builder dpmm(int dpmm) {
            if (dpmm != 6 && dpmm != 8 && dpmm != 12 && dpmm != 24) {
                throw new IllegalArgumentException("dpmm must be 6, 8, 12 or 24, got " + dpmm);
            }
            this.dpmm = dpmm;
            return this;
        }

        /**
         * Asks the server for a {@code Server-Timing} header with the duration of each stage.
         *
         * @param timings {@code true} to ask for timings
         * @return this builder
         */
        public Builder timings(boolean timings) {
            this.timings = timings;
            return this;
        }

        /**
         * Sets the {@code Idempotency-Key}, for example an order number. A retry with the same
         * key and the same content is billed once. Without one, the client generates a key per
         * call when retries are enabled.
         *
         * @param idempotencyKey a non-empty key of at most 255 characters
         * @return this builder
         * @throws IllegalArgumentException if the key is empty, longer than 255 characters or
         *     contains control characters
         */
        public Builder idempotencyKey(String idempotencyKey) {
            Objects.requireNonNull(idempotencyKey, "idempotencyKey");
            if (idempotencyKey.isEmpty() || idempotencyKey.length() > 255) {
                throw new IllegalArgumentException("idempotencyKey must be 1 to 255 characters");
            }
            for (int i = 0; i < idempotencyKey.length(); i++) {
                char c = idempotencyKey.charAt(i);
                if (c < 0x20 || c == 0x7f) {
                    throw new IllegalArgumentException("idempotencyKey must not contain control characters");
                }
            }
            this.idempotencyKey = idempotencyKey;
            return this;
        }

        /**
         * Gzips the request body and sends it with {@code Content-Encoding: gzip}. Useful for
         * large uploads on slow links.
         *
         * @param compress {@code true} to compress
         * @return this builder
         */
        public Builder compress(boolean compress) {
            this.compress = compress;
            return this;
        }

        /**
         * Builds the options.
         *
         * @return the options
         */
        public RenderOptions build() {
            return new RenderOptions(this);
        }
    }
}
