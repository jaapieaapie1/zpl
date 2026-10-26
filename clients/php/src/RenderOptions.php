<?php

declare(strict_types=1);

namespace Caretly;

use InvalidArgumentException;

/**
 * Options for a render request. Leave an option null to use the server default.
 */
final class RenderOptions
{
    /** Printer resolutions the server supports, in dots per millimetre. */
    public const DPMM_VALUES = [6, 8, 12, 24];

    /**
     * @param Format|null $format         Output format; the server defaults to PDF.
     * @param int|null    $page           For PNG: the page to render, starting at 1.
     * @param int|null    $dpmm           Printer resolution: 6, 8, 12 or 24; the server defaults to 8.
     * @param bool        $timings        Ask for a `Server-Timing` response header.
     * @param string|null $idempotencyKey Sent as `Idempotency-Key` (1 to 255 characters). A retried
     *                                    request with the same key and content is billed once. When
     *                                    null and retries are enabled, the client generates one per call.
     * @param bool        $compress       Gzip the request body (needs ext-zlib).
     *
     * @throws InvalidArgumentException when an option is out of range.
     */
    public function __construct(
        public readonly ?Format $format = null,
        public readonly ?int $page = null,
        public readonly ?int $dpmm = null,
        public readonly bool $timings = false,
        public readonly ?string $idempotencyKey = null,
        public readonly bool $compress = false,
    ) {
        if ($page !== null && $page < 1) {
            throw new InvalidArgumentException("page must be 1 or more, got {$page}");
        }
        if ($dpmm !== null && !in_array($dpmm, self::DPMM_VALUES, true)) {
            throw new InvalidArgumentException("dpmm must be 6, 8, 12 or 24, got {$dpmm}");
        }
        if ($idempotencyKey !== null && ($idempotencyKey === '' || strlen($idempotencyKey) > 255)) {
            throw new InvalidArgumentException('idempotencyKey must be 1 to 255 characters');
        }
        if ($compress && !function_exists('gzencode')) {
            throw new InvalidArgumentException('compress needs the zlib extension');
        }
    }

    /**
     * The query parameters for these options; unset options are left out.
     *
     * @return array<string, string>
     */
    public function toQuery(): array
    {
        $query = [];
        if ($this->format !== null) {
            $query['format'] = $this->format->value;
        }
        if ($this->page !== null) {
            $query['page'] = (string) $this->page;
        }
        if ($this->dpmm !== null) {
            $query['dpmm'] = (string) $this->dpmm;
        }
        if ($this->timings) {
            $query['timings'] = '1';
        }

        return $query;
    }
}
