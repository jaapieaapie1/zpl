<?php

declare(strict_types=1);

namespace Caretly\Exception;

/**
 * 413: the body is larger than 100 MiB after decompression.
 */
class PayloadTooLargeException extends ApiException
{
}
