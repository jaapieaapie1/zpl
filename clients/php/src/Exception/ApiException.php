<?php

declare(strict_types=1);

namespace Caretly\Exception;

use Throwable;

/**
 * The API answered with an error status. The message is the plain-text response body.
 */
class ApiException extends CaretlyException
{
    /**
     * @param int|null $retryAfter Seconds from the `Retry-After` header, when present.
     */
    public function __construct(
        string $message,
        public readonly int $status,
        public readonly ?int $retryAfter = null,
        ?Throwable $previous = null,
    ) {
        parent::__construct($message, $status, $previous);
    }

    /**
     * The exception class that fits a status code.
     */
    public static function fromStatus(int $status, string $message, ?int $retryAfter = null): self
    {
        if ($message === '') {
            $message = "HTTP {$status}";
        }

        return match ($status) {
            400 => new BadRequestException($message, $status, $retryAfter),
            401 => new AuthenticationException($message, $status, $retryAfter),
            402 => new PaymentRequiredException($message, $status, $retryAfter),
            413 => new PayloadTooLargeException($message, $status, $retryAfter),
            415 => new UnsupportedEncodingException($message, $status, $retryAfter),
            429 => new RateLimitedException($message, $status, $retryAfter),
            503 => new OverloadedException($message, $status, $retryAfter),
            default => new self($message, $status, $retryAfter),
        };
    }
}
