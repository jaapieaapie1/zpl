<?php

declare(strict_types=1);

namespace Caretly;

use Caretly\Exception\CaretlyException;

/**
 * A rendered PDF or PNG with the metadata from the response headers.
 */
final class RenderResult
{
    /**
     * @param string         $content        The file bytes.
     * @param string         $contentType    `application/pdf` or `image/png`.
     * @param int            $pageCount      Total pages across the input (`X-Page-Count`).
     * @param list<int>      $filePageCounts Pages per file for multipart requests, else empty.
     * @param int|null       $labelsBilled   Labels billed (0 for test keys); null when the server has no keys.
     * @param RateLimit|null $rateLimit      Rate limit state; null when the server sends none.
     * @param string|null    $serverTiming   Raw `Server-Timing` header, with the `timings` option.
     */
    public function __construct(
        public readonly string $content,
        public readonly string $contentType,
        public readonly int $pageCount,
        public readonly array $filePageCounts = [],
        public readonly ?int $labelsBilled = null,
        public readonly ?RateLimit $rateLimit = null,
        public readonly ?string $serverTiming = null,
    ) {
    }

    /**
     * Write the file bytes to a path, replacing any existing file.
     *
     * @throws CaretlyException when the file cannot be written.
     */
    public function saveTo(string $path): void
    {
        $written = @file_put_contents($path, $this->content);
        if ($written !== strlen($this->content)) {
            $error = error_get_last()['message'] ?? 'unknown error';
            throw new CaretlyException("could not write {$path}: {$error}");
        }
    }
}
