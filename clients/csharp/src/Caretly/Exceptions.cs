using System;

namespace Caretly
{
    /// <summary>The base class of every error thrown by the Caretly client.</summary>
    public class CaretlyException : Exception
    {
        /// <summary>Creates the exception.</summary>
        /// <param name="message">The error message.</param>
        public CaretlyException(string message)
            : base(message)
        {
        }

        /// <summary>Creates the exception with a cause.</summary>
        /// <param name="message">The error message.</param>
        /// <param name="innerException">The underlying error.</param>
        public CaretlyException(string message, Exception? innerException)
            : base(message, innerException)
        {
        }
    }

    /// <summary>The server could not be reached, or the connection failed.</summary>
    public class CaretlyConnectionException : CaretlyException
    {
        /// <summary>Creates the exception.</summary>
        /// <param name="message">The error message.</param>
        /// <param name="innerException">The underlying error.</param>
        public CaretlyConnectionException(string message, Exception? innerException)
            : base(message, innerException)
        {
        }
    }

    /// <summary>The request took longer than <see cref="CaretlyClientOptions.Timeout"/>.</summary>
    public class CaretlyTimeoutException : CaretlyException
    {
        /// <summary>Creates the exception.</summary>
        /// <param name="message">The error message.</param>
        /// <param name="innerException">The underlying error.</param>
        public CaretlyTimeoutException(string message, Exception? innerException)
            : base(message, innerException)
        {
        }
    }

    /// <summary>The server answered with an error status.</summary>
    public class CaretlyApiException : CaretlyException
    {
        /// <summary>Creates the exception.</summary>
        /// <param name="statusCode">The HTTP status code.</param>
        /// <param name="responseBody">The plain-text error message from the server.</param>
        /// <param name="retryAfter">The <c>Retry-After</c> delay, when the server sent one.</param>
        public CaretlyApiException(int statusCode, string responseBody, TimeSpan? retryAfter)
            : base(FormatMessage(statusCode, responseBody))
        {
            StatusCode = statusCode;
            ResponseBody = responseBody ?? string.Empty;
            RetryAfter = retryAfter;
        }

        /// <summary>The HTTP status code.</summary>
        public int StatusCode { get; }

        /// <summary>The plain-text error message from the server.</summary>
        public string ResponseBody { get; }

        /// <summary>The <c>Retry-After</c> delay, when the server sent one.</summary>
        public TimeSpan? RetryAfter { get; }

        private static string FormatMessage(int statusCode, string? body)
        {
            var text = body?.Trim();
            return string.IsNullOrEmpty(text)
                ? $"Caretly API error {statusCode}"
                : $"Caretly API error {statusCode}: {text}";
        }
    }

    /// <summary>400: invalid ZPL, invalid parameters or a page out of range.</summary>
    public class BadRequestException : CaretlyApiException
    {
        /// <inheritdoc cref="CaretlyApiException(int, string, TimeSpan?)" />
        public BadRequestException(int statusCode, string responseBody, TimeSpan? retryAfter)
            : base(statusCode, responseBody, retryAfter)
        {
        }
    }

    /// <summary>401: the API key is missing, not valid, expired or revoked.</summary>
    public class AuthenticationException : CaretlyApiException
    {
        /// <inheritdoc cref="CaretlyApiException(int, string, TimeSpan?)" />
        public AuthenticationException(int statusCode, string responseBody, TimeSpan? retryAfter)
            : base(statusCode, responseBody, retryAfter)
        {
        }
    }

    /// <summary>402: billing for the account is not active, so live keys are paused.</summary>
    public class PaymentRequiredException : CaretlyApiException
    {
        /// <inheritdoc cref="CaretlyApiException(int, string, TimeSpan?)" />
        public PaymentRequiredException(int statusCode, string responseBody, TimeSpan? retryAfter)
            : base(statusCode, responseBody, retryAfter)
        {
        }
    }

    /// <summary>413: the body is larger than 100 MiB after decompression.</summary>
    public class PayloadTooLargeException : CaretlyApiException
    {
        /// <inheritdoc cref="CaretlyApiException(int, string, TimeSpan?)" />
        public PayloadTooLargeException(int statusCode, string responseBody, TimeSpan? retryAfter)
            : base(statusCode, responseBody, retryAfter)
        {
        }
    }

    /// <summary>415: the server does not support the request's <c>Content-Encoding</c>.</summary>
    public class UnsupportedEncodingException : CaretlyApiException
    {
        /// <inheritdoc cref="CaretlyApiException(int, string, TimeSpan?)" />
        public UnsupportedEncodingException(int statusCode, string responseBody, TimeSpan? retryAfter)
            : base(statusCode, responseBody, retryAfter)
        {
        }
    }

    /// <summary>429: too many requests for this API key. Retried automatically.</summary>
    public class RateLimitedException : CaretlyApiException
    {
        /// <inheritdoc cref="CaretlyApiException(int, string, TimeSpan?)" />
        public RateLimitedException(int statusCode, string responseBody, TimeSpan? retryAfter)
            : base(statusCode, responseBody, retryAfter)
        {
        }
    }

    /// <summary>503: the renderer is busy and turned the request away. Not billed; retried automatically.</summary>
    public class OverloadedException : CaretlyApiException
    {
        /// <inheritdoc cref="CaretlyApiException(int, string, TimeSpan?)" />
        public OverloadedException(int statusCode, string responseBody, TimeSpan? retryAfter)
            : base(statusCode, responseBody, retryAfter)
        {
        }
    }
}
