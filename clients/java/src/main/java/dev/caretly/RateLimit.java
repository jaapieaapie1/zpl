package dev.caretly;

/**
 * The rate limit of an API key, from the {@code RateLimit-Limit} and {@code RateLimit-Remaining}
 * response headers.
 */
public final class RateLimit {
    private final int limit;
    private final int remaining;

    /**
     * Creates a rate limit.
     *
     * @param limit the burst size: how many requests the key can make at once
     * @param remaining the requests left in the burst
     */
    public RateLimit(int limit, int remaining) {
        this.limit = limit;
        this.remaining = remaining;
    }

    /**
     * Returns the burst size: how many requests the key can make at once.
     *
     * @return the burst size
     */
    public int limit() {
        return limit;
    }

    /**
     * Returns the requests the key can still make at once before it is rate limited.
     *
     * @return the requests left in the burst
     */
    public int remaining() {
        return remaining;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (!(o instanceof RateLimit)) {
            return false;
        }
        RateLimit other = (RateLimit) o;
        return limit == other.limit && remaining == other.remaining;
    }

    @Override
    public int hashCode() {
        return 31 * limit + remaining;
    }

    @Override
    public String toString() {
        return "RateLimit{limit=" + limit + ", remaining=" + remaining + "}";
    }
}
