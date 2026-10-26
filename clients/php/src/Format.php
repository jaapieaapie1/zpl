<?php

declare(strict_types=1);

namespace Caretly;

/**
 * Output format of a render.
 */
enum Format: string
{
    /** A vector PDF with every page. */
    case Pdf = 'pdf';

    /** A PNG of one page, chosen with the `page` option. */
    case Png = 'png';

    public function contentType(): string
    {
        return match ($this) {
            self::Pdf => 'application/pdf',
            self::Png => 'image/png',
        };
    }
}
