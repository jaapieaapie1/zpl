<?php

declare(strict_types=1);

namespace Caretly\Exception;

/**
 * 402: billing for the account is not active, so live keys are paused.
 */
class PaymentRequiredException extends ApiException
{
}
