package caretly

import (
	"context"
	"errors"
	"fmt"
	"net"
	"net/http"
	"strconv"
	"strings"
	"time"
)

// Sentinel errors for use with errors.Is. An *APIError matches ErrAPI and the
// sentinel for its status code; transport failures match ErrConnection or
// ErrTimeout.
var (
	// ErrAPI matches every *APIError, whatever its status code.
	ErrAPI = errors.New("caretly: API error")
	// ErrBadRequest matches a 400: a ZPL parse error, a bad parameter or a
	// page out of range.
	ErrBadRequest = errors.New("caretly: bad request")
	// ErrUnauthorized matches a 401: the API key is missing, not valid,
	// expired or revoked.
	ErrUnauthorized = errors.New("caretly: unauthorized")
	// ErrPaymentRequired matches a 402: billing is not active, so live keys
	// are paused. Test keys keep working.
	ErrPaymentRequired = errors.New("caretly: payment required")
	// ErrPayloadTooLarge matches a 413: the body is over 100 MiB after
	// decompression.
	ErrPayloadTooLarge = errors.New("caretly: payload too large")
	// ErrUnsupportedEncoding matches a 415: the Content-Encoding is not
	// supported.
	ErrUnsupportedEncoding = errors.New("caretly: unsupported content encoding")
	// ErrRateLimited matches a 429. APIError.RetryAfter says how long to wait.
	ErrRateLimited = errors.New("caretly: rate limited")
	// ErrOverloaded matches a 503: the server is shedding load. Shed requests
	// are not billed. APIError.RetryAfter says how long to wait.
	ErrOverloaded = errors.New("caretly: server overloaded")

	// ErrConnection matches a transport failure: the request did not reach
	// the server or the response could not be read.
	ErrConnection = errors.New("caretly: connection error")
	// ErrTimeout matches an attempt that ran over the client timeout.
	ErrTimeout = errors.New("caretly: request timed out")

	// ErrInvalidOptions matches options rejected before any request is sent.
	ErrInvalidOptions = errors.New("caretly: invalid options")
)

// APIError is returned when the server answers with a status other than 200.
type APIError struct {
	// StatusCode is the HTTP status code, for example 429.
	StatusCode int
	// Message is the plain-text response body.
	Message string
	// RetryAfter is the wait the server asked for in its Retry-After header,
	// or zero when the header was absent.
	RetryAfter time.Duration
}

func (e *APIError) Error() string {
	return fmt.Sprintf("caretly: %d %s: %s", e.StatusCode, http.StatusText(e.StatusCode), e.Message)
}

// Is reports whether target is ErrAPI or the sentinel for e's status code.
func (e *APIError) Is(target error) bool {
	if target == ErrAPI {
		return true
	}
	s := statusSentinel(e.StatusCode)
	return s != nil && target == s
}

func statusSentinel(status int) error {
	switch status {
	case http.StatusBadRequest:
		return ErrBadRequest
	case http.StatusUnauthorized:
		return ErrUnauthorized
	case http.StatusPaymentRequired:
		return ErrPaymentRequired
	case http.StatusRequestEntityTooLarge:
		return ErrPayloadTooLarge
	case http.StatusUnsupportedMediaType:
		return ErrUnsupportedEncoding
	case http.StatusTooManyRequests:
		return ErrRateLimited
	case http.StatusServiceUnavailable:
		return ErrOverloaded
	}
	return nil
}

func newAPIError(resp *http.Response, body []byte) *APIError {
	msg := strings.TrimSpace(string(body))
	if msg == "" {
		msg = http.StatusText(resp.StatusCode)
	}
	return &APIError{
		StatusCode: resp.StatusCode,
		Message:    msg,
		RetryAfter: parseRetryAfter(resp.Header.Get("Retry-After")),
	}
}

// parseRetryAfter reads delay-seconds or an HTTP date; zero when absent.
func parseRetryAfter(v string) time.Duration {
	v = strings.TrimSpace(v)
	if v == "" {
		return 0
	}
	if secs, err := strconv.Atoi(v); err == nil {
		if secs < 0 {
			return 0
		}
		return time.Duration(secs) * time.Second
	}
	if t, err := http.ParseTime(v); err == nil {
		if d := time.Until(t); d > 0 {
			return d
		}
	}
	return 0
}

// TransportError is returned when a request fails before an HTTP response
// is received, or the response cannot be read. It matches ErrTimeout when
// the attempt ran over the client timeout and ErrConnection otherwise.
type TransportError struct {
	// Timeout is true when the attempt ran over the client timeout.
	Timeout bool
	// Err is the underlying error from net/http.
	Err error
}

func (e *TransportError) Error() string {
	if e.Timeout {
		return "caretly: request timed out: " + e.Err.Error()
	}
	return "caretly: connection error: " + e.Err.Error()
}

func (e *TransportError) Unwrap() error { return e.Err }

// Is reports whether target is ErrTimeout or ErrConnection, matching e.
func (e *TransportError) Is(target error) bool {
	if e.Timeout {
		return target == ErrTimeout
	}
	return target == ErrConnection
}

// transportError classifies err from an attempt. When the caller's ctx is
// done, its error is returned as is so errors.Is(err, context.Canceled) works.
func transportError(ctx, attemptCtx context.Context, err error) error {
	if ctx.Err() != nil {
		return ctx.Err()
	}
	timeout := errors.Is(attemptCtx.Err(), context.DeadlineExceeded)
	var ne net.Error
	if errors.As(err, &ne) && ne.Timeout() {
		timeout = true
	}
	return &TransportError{Timeout: timeout, Err: err}
}
