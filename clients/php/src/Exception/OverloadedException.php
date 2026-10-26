<?php

declare(strict_types=1);

namespace Caretly\Exception;

/**
 * 503: the server is overloaded and shed the request (not billed). See retryAfter.
 */
class OverloadedException extends ApiException
{
}
