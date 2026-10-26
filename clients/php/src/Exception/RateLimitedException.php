<?php

declare(strict_types=1);

namespace Caretly\Exception;

/**
 * 429: too many requests for this API key. See retryAfter.
 */
class RateLimitedException extends ApiException
{
}
