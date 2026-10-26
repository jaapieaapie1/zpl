<?php

declare(strict_types=1);

namespace Caretly;

/**
 * Rate limit state of the API key, from the `RateLimit-*` response headers.
 */
final class RateLimit
{
    /**
     * @param int|null $limit     Burst size: how many requests the key can make at once.
     * @param int|null $remaining Requests left in the burst.
     */
    public function __construct(
        public readonly ?int $limit,
        public readonly ?int $remaining,
    ) {
    }
}
