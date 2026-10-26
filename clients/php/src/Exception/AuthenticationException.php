<?php

declare(strict_types=1);

namespace Caretly\Exception;

/**
 * 401: the API key is missing, not valid, expired or revoked.
 */
class AuthenticationException extends ApiException
{
}
