package dev.caretly;

/** The output format of a render. */
public enum Format {
    /** A PDF with every page. */
    PDF("pdf", "application/pdf"),
    /** A PNG of a single page, chosen with {@link RenderOptions.Builder#page(int)}. */
    PNG("png", "image/png");

    private final String value;
    private final String contentType;

    Format(String value, String contentType) {
        this.value = value;
        this.contentType = contentType;
    }

    /**
     * Returns the value sent as the {@code format} query parameter.
     *
     * @return {@code "pdf"} or {@code "png"}
     */
    public String value() {
        return value;
    }

    /**
     * Returns the media type of a response in this format.
     *
     * @return {@code "application/pdf"} or {@code "image/png"}
     */
    public String contentType() {
        return contentType;
    }
}
