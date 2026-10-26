package caretly

import (
	"context"
	"io"
	"net/http"
	"os"
	"strings"
	"time"
)

// Version is the version of this client library.
const Version = "0.1.0"

// DefaultBaseURL is the Caretly API used when no base URL is configured.
const DefaultBaseURL = "https://caretly.dev"

// DefaultTimeout is the time limit for a single HTTP attempt.
const DefaultTimeout = 30 * time.Second

// DefaultMaxRetries is how many times a rate limited, overloaded or failed
// connection is retried before the error is returned.
const DefaultMaxRetries = 2

// APIKeyEnv is the environment variable read when no API key is given.
const APIKeyEnv = "CARETLY_API_KEY"

const userAgent = "caretly-go/" + Version

// Client talks to the Caretly API. It is safe for concurrent use.
type Client struct {
	apiKey     string
	baseURL    string
	httpClient *http.Client
	timeout    time.Duration
	maxRetries int

	// sleep waits for d or until ctx is done; replaced in tests.
	sleep func(ctx context.Context, d time.Duration) error
}

// Option configures a Client.
type Option func(*Client)

// WithAPIKey sets the API key sent as "Authorization: Bearer <key>".
// Without this option the CARETLY_API_KEY environment variable is used.
// An empty key sends no Authorization header, for self-hosted servers.
func WithAPIKey(key string) Option {
	return func(c *Client) { c.apiKey = key }
}

// WithBaseURL sets the API base URL, for example a self-hosted server.
// The default is https://caretly.dev.
func WithBaseURL(baseURL string) Option {
	return func(c *Client) { c.baseURL = strings.TrimRight(baseURL, "/") }
}

// WithHTTPClient sets the HTTP client used for requests. The default is a
// new http.Client. The timeout from WithTimeout applies on top of it.
func WithHTTPClient(hc *http.Client) Option {
	return func(c *Client) {
		if hc != nil {
			c.httpClient = hc
		}
	}
}

// WithTimeout sets the time limit for each HTTP attempt (default 30s).
// Zero or a negative value disables the limit. The context passed to a
// method still applies to the whole call, retries included.
func WithTimeout(d time.Duration) Option {
	return func(c *Client) { c.timeout = d }
}

// WithMaxRetries sets how many times a request is retried after a 429, a 503
// or a connection error (default 2). Zero disables retries.
func WithMaxRetries(n int) Option {
	return func(c *Client) {
		if n < 0 {
			n = 0
		}
		c.maxRetries = n
	}
}

// NewClient returns a Client configured with opts.
func NewClient(opts ...Option) *Client {
	c := &Client{
		apiKey:     os.Getenv(APIKeyEnv),
		baseURL:    DefaultBaseURL,
		httpClient: &http.Client{},
		timeout:    DefaultTimeout,
		maxRetries: DefaultMaxRetries,
		sleep:      sleepContext,
	}
	for _, opt := range opts {
		opt(c)
	}
	return c
}

// Health reports whether the server is up: it calls GET /health and returns
// true when the server answers 200 with "ok". It is not retried.
func (c *Client) Health(ctx context.Context) (bool, error) {
	actx, cancel := c.attemptContext(ctx)
	defer cancel()
	req, err := http.NewRequestWithContext(actx, http.MethodGet, c.baseURL+"/health", nil)
	if err != nil {
		return false, err
	}
	req.Header.Set("User-Agent", userAgent)
	resp, err := c.httpClient.Do(req)
	if err != nil {
		return false, transportError(ctx, actx, err)
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(io.LimitReader(resp.Body, 1024))
	if err != nil {
		return false, transportError(ctx, actx, err)
	}
	if resp.StatusCode != http.StatusOK {
		return false, newAPIError(resp, body)
	}
	return strings.TrimSpace(string(body)) == "ok", nil
}

func (c *Client) attemptContext(ctx context.Context) (context.Context, context.CancelFunc) {
	if c.timeout > 0 {
		return context.WithTimeout(ctx, c.timeout)
	}
	return context.WithCancel(ctx)
}

func sleepContext(ctx context.Context, d time.Duration) error {
	t := time.NewTimer(d)
	defer t.Stop()
	select {
	case <-ctx.Done():
		return ctx.Err()
	case <-t.C:
		return nil
	}
}
