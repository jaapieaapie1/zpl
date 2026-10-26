package caretly

import (
	"compress/gzip"
	"context"
	"errors"
	"io"
	"mime"
	"mime/multipart"
	"net"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"sync"
	"testing"
	"time"
)

// recorder collects the requests a test server receives.
type recorder struct {
	mu       sync.Mutex
	requests []*http.Request
	bodies   [][]byte
}

func (r *recorder) add(req *http.Request, body []byte) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.requests = append(r.requests, req)
	r.bodies = append(r.bodies, body)
}

func (r *recorder) count() int {
	r.mu.Lock()
	defer r.mu.Unlock()
	return len(r.requests)
}

// newServer starts a server whose handler sees the request number (from 0).
func newServer(t *testing.T, handler func(n int, w http.ResponseWriter, r *http.Request)) (*httptest.Server, *recorder) {
	t.Helper()
	rec := &recorder{}
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		body, _ := io.ReadAll(r.Body)
		rec.mu.Lock()
		n := len(rec.requests)
		rec.mu.Unlock()
		rec.add(r, body)
		handler(n, w, r)
	}))
	t.Cleanup(srv.Close)
	return srv, rec
}

// testClient returns a client for srv that records sleeps instead of waiting.
func testClient(srv *httptest.Server, opts ...Option) (*Client, *[]time.Duration) {
	var sleeps []time.Duration
	c := NewClient(append([]Option{WithAPIKey("zpl_test_key"), WithBaseURL(srv.URL)}, opts...)...)
	c.sleep = func(ctx context.Context, d time.Duration) error {
		sleeps = append(sleeps, d)
		return ctx.Err()
	}
	return c, &sleeps
}

func ok(w http.ResponseWriter) {
	w.Header().Set("Content-Type", "application/pdf")
	w.Header().Set("X-Page-Count", "1")
	_, _ = io.WriteString(w, "%PDF")
}

func TestRenderSuccessHeaders(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) {
		h := w.Header()
		h.Set("Content-Type", "image/png")
		h.Set("X-Page-Count", "3")
		h.Set("X-File-Page-Counts", "2, 1")
		h.Set("X-Labels-Billed", "1")
		h.Set("RateLimit-Limit", "200")
		h.Set("RateLimit-Remaining", "199")
		h.Set("Server-Timing", "read;dur=0.1, parse;dur=0.2")
		_, _ = io.WriteString(w, "PNGDATA")
	})
	c, _ := testClient(srv)
	res, err := c.Render(context.Background(), "^XA^XZ", &RenderOptions{Format: FormatPNG, Page: 2, DPMM: 12, Timings: true})
	if err != nil {
		t.Fatal(err)
	}
	if string(res.Content) != "PNGDATA" || res.ContentType != "image/png" || res.PageCount != 3 {
		t.Errorf("unexpected result %+v", res)
	}
	if len(res.FilePageCounts) != 2 || res.FilePageCounts[0] != 2 || res.FilePageCounts[1] != 1 {
		t.Errorf("FilePageCounts = %v", res.FilePageCounts)
	}
	if res.LabelsBilled == nil || *res.LabelsBilled != 1 {
		t.Errorf("LabelsBilled = %v", res.LabelsBilled)
	}
	if res.RateLimit == nil || *res.RateLimit != (RateLimit{Limit: 200, Remaining: 199}) {
		t.Errorf("RateLimit = %v", res.RateLimit)
	}
	if res.ServerTiming != "read;dur=0.1, parse;dur=0.2" {
		t.Errorf("ServerTiming = %q", res.ServerTiming)
	}

	req := rec.requests[0]
	if req.Method != http.MethodPost || req.URL.Path != "/render" {
		t.Errorf("request %s %s", req.Method, req.URL.Path)
	}
	q := req.URL.Query()
	if q.Get("format") != "png" || q.Get("page") != "2" || q.Get("dpmm") != "12" || q.Get("timings") != "1" {
		t.Errorf("query = %s", req.URL.RawQuery)
	}
	if got := req.Header.Get("Content-Type"); got != "text/plain; charset=utf-8" {
		t.Errorf("Content-Type = %q", got)
	}
	if got := req.Header.Get("User-Agent"); got != "caretly-go/"+Version {
		t.Errorf("User-Agent = %q", got)
	}
	if string(rec.bodies[0]) != "^XA^XZ" {
		t.Errorf("body = %q", rec.bodies[0])
	}
}

func TestRenderOptionalHeadersAbsent(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) { ok(w) })
	c, _ := testClient(srv)
	res, err := c.Render(context.Background(), "^XA^XZ", nil)
	if err != nil {
		t.Fatal(err)
	}
	if res.LabelsBilled != nil || res.RateLimit != nil || res.ServerTiming != "" {
		t.Errorf("expected optional fields unset: %+v", res)
	}
	if res.FilePageCounts == nil || len(res.FilePageCounts) != 0 {
		t.Errorf("FilePageCounts = %#v, want empty", res.FilePageCounts)
	}
	if raw := rec.requests[0].URL.RawQuery; raw != "" {
		t.Errorf("query = %q, want none so server defaults apply", raw)
	}
}

func TestAuthHeader(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) { ok(w) })

	c, _ := testClient(srv, WithAPIKey("zpl_live_abc"))
	if _, err := c.Render(context.Background(), "^XA^XZ", nil); err != nil {
		t.Fatal(err)
	}
	if got := rec.requests[0].Header.Get("Authorization"); got != "Bearer zpl_live_abc" {
		t.Errorf("Authorization = %q", got)
	}

	c, _ = testClient(srv, WithAPIKey(""))
	if _, err := c.Render(context.Background(), "^XA^XZ", nil); err != nil {
		t.Fatal(err)
	}
	if got, present := rec.requests[1].Header["Authorization"]; present {
		t.Errorf("Authorization sent without a key: %q", got)
	}
}

func TestAPIKeyFromEnv(t *testing.T) {
	t.Setenv(APIKeyEnv, "zpl_test_env")
	if c := NewClient(); c.apiKey != "zpl_test_env" {
		t.Errorf("apiKey = %q", c.apiKey)
	}
	if c := NewClient(WithAPIKey("explicit")); c.apiKey != "explicit" {
		t.Errorf("apiKey = %q", c.apiKey)
	}
}

func TestDefaults(t *testing.T) {
	t.Setenv(APIKeyEnv, "")
	c := NewClient()
	if c.baseURL != DefaultBaseURL || c.timeout != 30*time.Second || c.maxRetries != 2 || c.apiKey != "" {
		t.Errorf("defaults: %+v", c)
	}
	if c := NewClient(WithBaseURL("http://localhost:3000/")); c.baseURL != "http://localhost:3000" {
		t.Errorf("baseURL = %q", c.baseURL)
	}
}

func TestErrorStatusMapping(t *testing.T) {
	cases := []struct {
		status   int
		sentinel error
	}{
		{400, ErrBadRequest},
		{401, ErrUnauthorized},
		{402, ErrPaymentRequired},
		{413, ErrPayloadTooLarge},
		{415, ErrUnsupportedEncoding},
		{429, ErrRateLimited},
		{503, ErrOverloaded},
		{500, nil},
		{404, nil},
	}
	all := []error{ErrBadRequest, ErrUnauthorized, ErrPaymentRequired, ErrPayloadTooLarge,
		ErrUnsupportedEncoding, ErrRateLimited, ErrOverloaded, ErrConnection, ErrTimeout}
	for _, tc := range cases {
		srv, _ := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) {
			w.Header().Set("Retry-After", "7")
			w.WriteHeader(tc.status)
			_, _ = io.WriteString(w, "message body\n")
		})
		c, _ := testClient(srv, WithMaxRetries(0))
		_, err := c.Render(context.Background(), "^XA^XZ", nil)
		var apiErr *APIError
		if !errors.As(err, &apiErr) {
			t.Fatalf("%d: error %v is not *APIError", tc.status, err)
		}
		if apiErr.StatusCode != tc.status || apiErr.Message != "message body" || apiErr.RetryAfter != 7*time.Second {
			t.Errorf("%d: %+v", tc.status, apiErr)
		}
		if !errors.Is(err, ErrAPI) {
			t.Errorf("%d: not ErrAPI", tc.status)
		}
		for _, s := range all {
			if want := s == tc.sentinel; errors.Is(err, s) != want {
				t.Errorf("%d: errors.Is(%v) = %v, want %v", tc.status, s, !want, want)
			}
		}
	}
}

func TestEmptyErrorBodyUsesStatusText(t *testing.T) {
	srv, _ := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	})
	c, _ := testClient(srv)
	_, err := c.Render(context.Background(), "^XA^XZ", nil)
	var apiErr *APIError
	if !errors.As(err, &apiErr) || apiErr.Message != "Internal Server Error" || apiErr.RetryAfter != 0 {
		t.Fatalf("err = %#v", err)
	}
}

func TestRetryOn429HonorsRetryAfter(t *testing.T) {
	srv, rec := newServer(t, func(n int, w http.ResponseWriter, _ *http.Request) {
		if n < 2 {
			w.Header().Set("Retry-After", "3")
			w.WriteHeader(http.StatusTooManyRequests)
			_, _ = io.WriteString(w, "Too many requests")
			return
		}
		ok(w)
	})
	c, sleeps := testClient(srv)
	if _, err := c.Render(context.Background(), "^XA^XZ", nil); err != nil {
		t.Fatal(err)
	}
	if rec.count() != 3 {
		t.Fatalf("requests = %d, want 3", rec.count())
	}
	if len(*sleeps) != 2 || (*sleeps)[0] != 3*time.Second || (*sleeps)[1] != 3*time.Second {
		t.Errorf("sleeps = %v, want [3s 3s]", *sleeps)
	}
	key := rec.requests[0].Header.Get("Idempotency-Key")
	uuidV4 := regexp.MustCompile(`^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$`)
	if !uuidV4.MatchString(key) {
		t.Errorf("Idempotency-Key %q is not a UUID v4", key)
	}
	for i, r := range rec.requests {
		if got := r.Header.Get("Idempotency-Key"); got != key {
			t.Errorf("request %d Idempotency-Key = %q, want %q", i, got, key)
		}
		if string(rec.bodies[i]) != "^XA^XZ" {
			t.Errorf("request %d body = %q", i, rec.bodies[i])
		}
	}
}

func TestRetryOn503WithBackoff(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusServiceUnavailable)
		_, _ = io.WriteString(w, "overloaded")
	})
	c, sleeps := testClient(srv, WithMaxRetries(3))
	_, err := c.Render(context.Background(), "^XA^XZ", nil)
	if !errors.Is(err, ErrOverloaded) {
		t.Fatalf("err = %v", err)
	}
	if rec.count() != 4 {
		t.Errorf("requests = %d, want 4", rec.count())
	}
	want := []time.Duration{500 * time.Millisecond, time.Second, 2 * time.Second}
	if len(*sleeps) != len(want) {
		t.Fatalf("sleeps = %v, want %v", *sleeps, want)
	}
	for i := range want {
		if (*sleeps)[i] != want[i] {
			t.Errorf("sleeps = %v, want %v", *sleeps, want)
		}
	}
}

func TestBackoffCap(t *testing.T) {
	want := []time.Duration{500 * time.Millisecond, time.Second, 2 * time.Second, 4 * time.Second, 8 * time.Second, 8 * time.Second}
	for i, w := range want {
		if got := backoff(i); got != w {
			t.Errorf("backoff(%d) = %v, want %v", i, got, w)
		}
	}
}

func TestNoRetryOnOtherStatuses(t *testing.T) {
	for _, status := range []int{400, 401, 402, 413, 415, 500, 502} {
		srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) {
			w.WriteHeader(status)
		})
		c, sleeps := testClient(srv)
		if _, err := c.Render(context.Background(), "^XA^XZ", nil); err == nil {
			t.Fatalf("%d: no error", status)
		}
		if rec.count() != 1 || len(*sleeps) != 0 {
			t.Errorf("%d: requests = %d, sleeps = %v", status, rec.count(), *sleeps)
		}
	}
}

func TestUserIdempotencyKeyKept(t *testing.T) {
	srv, rec := newServer(t, func(n int, w http.ResponseWriter, _ *http.Request) {
		if n == 0 {
			w.WriteHeader(http.StatusTooManyRequests)
			return
		}
		ok(w)
	})
	c, _ := testClient(srv)
	if _, err := c.Render(context.Background(), "^XA^XZ", &RenderOptions{IdempotencyKey: "order-42"}); err != nil {
		t.Fatal(err)
	}
	for i, r := range rec.requests {
		if got := r.Header.Get("Idempotency-Key"); got != "order-42" {
			t.Errorf("request %d Idempotency-Key = %q", i, got)
		}
	}
}

func TestNoIdempotencyKeyWithoutRetries(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) { ok(w) })
	c, _ := testClient(srv, WithMaxRetries(0))
	if _, err := c.Render(context.Background(), "^XA^XZ", nil); err != nil {
		t.Fatal(err)
	}
	if _, present := rec.requests[0].Header["Idempotency-Key"]; present {
		t.Error("Idempotency-Key sent with retries disabled")
	}
}

func TestContextCancelledDuringBackoff(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Retry-After", "60")
		w.WriteHeader(http.StatusTooManyRequests)
	})
	c := NewClient(WithBaseURL(srv.URL)) // real sleep
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()
	start := time.Now()
	_, err := c.Render(ctx, "^XA^XZ", nil)
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("err = %v, want context.DeadlineExceeded", err)
	}
	if elapsed := time.Since(start); elapsed > 5*time.Second {
		t.Errorf("Render waited %v despite the cancelled context", elapsed)
	}
	if rec.count() != 1 {
		t.Errorf("requests = %d, want 1", rec.count())
	}
}

func TestConnectionErrorRetried(t *testing.T) {
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatal(err)
	}
	addr := ln.Addr().String()
	ln.Close() // nothing listens: connections are refused

	c := NewClient(WithBaseURL("http://" + addr))
	var sleeps []time.Duration
	c.sleep = func(_ context.Context, d time.Duration) error {
		sleeps = append(sleeps, d)
		return nil
	}
	_, err = c.Render(context.Background(), "^XA^XZ", nil)
	if !errors.Is(err, ErrConnection) || errors.Is(err, ErrTimeout) || errors.Is(err, ErrAPI) {
		t.Fatalf("err = %v, want ErrConnection", err)
	}
	var te *TransportError
	if !errors.As(err, &te) || te.Err == nil {
		t.Errorf("err = %#v, want *TransportError", err)
	}
	if len(sleeps) != 2 {
		t.Errorf("sleeps = %v, want 2 retries", sleeps)
	}
}

func TestTimeout(t *testing.T) {
	release := make(chan struct{})
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, r *http.Request) {
		select {
		case <-release:
		case <-r.Context().Done():
		}
	})
	defer close(release)
	c, _ := testClient(srv, WithTimeout(50*time.Millisecond))
	_, err := c.Render(context.Background(), "^XA^XZ", nil)
	if !errors.Is(err, ErrTimeout) || errors.Is(err, ErrConnection) {
		t.Fatalf("err = %v, want ErrTimeout", err)
	}
	if rec.count() != 1 {
		t.Errorf("requests = %d, want 1 (timeouts are not retried)", rec.count())
	}
}

func TestRenderFilesMultipart(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("X-File-Page-Counts", "1, 2")
		ok(w)
	})
	c, _ := testClient(srv)
	res, err := c.RenderFiles(context.Background(), []File{
		{Name: "a.zpl", Content: []byte("^XA^FDa^FS^XZ")},
		{Name: "b.zpl", Content: []byte("^XA^FDb^FS^XZ^XA^XZ")},
	}, &RenderOptions{DPMM: 6})
	if err != nil {
		t.Fatal(err)
	}
	if len(res.FilePageCounts) != 2 || res.FilePageCounts[1] != 2 {
		t.Errorf("FilePageCounts = %v", res.FilePageCounts)
	}

	req := rec.requests[0]
	if req.URL.Query().Get("dpmm") != "6" {
		t.Errorf("query = %s", req.URL.RawQuery)
	}
	mediaType, params, err := mime.ParseMediaType(req.Header.Get("Content-Type"))
	if err != nil || mediaType != "multipart/form-data" {
		t.Fatalf("Content-Type = %q", req.Header.Get("Content-Type"))
	}
	mr := multipart.NewReader(strings.NewReader(string(rec.bodies[0])), params["boundary"])
	want := []struct{ name, body string }{{"a.zpl", "^XA^FDa^FS^XZ"}, {"b.zpl", "^XA^FDb^FS^XZ^XA^XZ"}}
	for i := 0; ; i++ {
		p, err := mr.NextPart()
		if err == io.EOF {
			if i != len(want) {
				t.Errorf("parts = %d, want %d", i, len(want))
			}
			break
		}
		if err != nil {
			t.Fatal(err)
		}
		if i >= len(want) {
			t.Fatalf("extra part %q", p.FileName())
		}
		body, _ := io.ReadAll(p)
		if p.FormName() != "file" || p.FileName() != want[i].name || string(body) != want[i].body {
			t.Errorf("part %d: name=%q file=%q body=%q", i, p.FormName(), p.FileName(), body)
		}
	}
}

func TestRenderFilesEmpty(t *testing.T) {
	c := NewClient()
	if _, err := c.RenderFiles(context.Background(), nil, nil); !errors.Is(err, ErrInvalidOptions) {
		t.Errorf("err = %v", err)
	}
}

func TestCompress(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) { ok(w) })
	c, _ := testClient(srv)
	zpl := strings.Repeat("^XA^FO10,10^FDcompress me^FS^XZ\n", 100)
	if _, err := c.Render(context.Background(), zpl, &RenderOptions{Compress: true}); err != nil {
		t.Fatal(err)
	}
	req := rec.requests[0]
	if got := req.Header.Get("Content-Encoding"); got != "gzip" {
		t.Errorf("Content-Encoding = %q", got)
	}
	if got := req.Header.Get("Content-Type"); got != "text/plain; charset=utf-8" {
		t.Errorf("Content-Type = %q", got)
	}
	zr, err := gzip.NewReader(strings.NewReader(string(rec.bodies[0])))
	if err != nil {
		t.Fatal(err)
	}
	plain, err := io.ReadAll(zr)
	if err != nil || string(plain) != zpl {
		t.Errorf("decompressed body mismatch (err %v)", err)
	}
	if len(rec.bodies[0]) >= len(zpl) {
		t.Errorf("compressed %d bytes to %d", len(zpl), len(rec.bodies[0]))
	}
}

func TestNoContentEncodingByDefault(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) { ok(w) })
	c, _ := testClient(srv)
	if _, err := c.Render(context.Background(), "^XA^XZ", nil); err != nil {
		t.Fatal(err)
	}
	if _, present := rec.requests[0].Header["Content-Encoding"]; present {
		t.Error("Content-Encoding sent without Compress")
	}
}

func TestValidation(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) { ok(w) })
	c, _ := testClient(srv)
	bad := []*RenderOptions{
		{Page: -1},
		{DPMM: 7},
		{DPMM: 300},
		{Format: "jpg"},
		{IdempotencyKey: strings.Repeat("k", 256)},
	}
	for _, o := range bad {
		if _, err := c.Render(context.Background(), "^XA^XZ", o); !errors.Is(err, ErrInvalidOptions) {
			t.Errorf("%+v: err = %v, want ErrInvalidOptions", o, err)
		}
	}
	if _, err := c.RenderPNG(context.Background(), "^XA^XZ", -2); !errors.Is(err, ErrInvalidOptions) {
		t.Errorf("RenderPNG(-2): err = %v", err)
	}
	if rec.count() != 0 {
		t.Errorf("invalid options sent %d requests", rec.count())
	}
	for _, d := range []int{6, 8, 12, 24} {
		if _, err := c.Render(context.Background(), "^XA^XZ", &RenderOptions{DPMM: d}); err != nil {
			t.Errorf("dpmm %d: %v", d, err)
		}
	}
}

func TestSugar(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) { ok(w) })
	c, _ := testClient(srv)
	if _, err := c.RenderPDF(context.Background(), "^XA^XZ"); err != nil {
		t.Fatal(err)
	}
	if _, err := c.RenderPNG(context.Background(), "^XA^XZ", 3); err != nil {
		t.Fatal(err)
	}
	if q := rec.requests[0].URL.RawQuery; q != "format=pdf" {
		t.Errorf("RenderPDF query = %q", q)
	}
	if q := rec.requests[1].URL.RawQuery; q != "format=png&page=3" {
		t.Errorf("RenderPNG query = %q", q)
	}
}

func TestSave(t *testing.T) {
	path := filepath.Join(t.TempDir(), "label.pdf")
	r := &RenderResult{Content: []byte("%PDF")}
	if err := r.Save(path); err != nil {
		t.Fatal(err)
	}
	got, err := os.ReadFile(path)
	if err != nil || string(got) != "%PDF" {
		t.Errorf("saved %q, %v", got, err)
	}
}

func TestHealth(t *testing.T) {
	srv, rec := newServer(t, func(_ int, w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/health" || r.Method != http.MethodGet {
			w.WriteHeader(http.StatusNotFound)
			return
		}
		_, _ = io.WriteString(w, "ok")
	})
	c, _ := testClient(srv)
	healthy, err := c.Health(context.Background())
	if err != nil || !healthy {
		t.Fatalf("Health = %v, %v", healthy, err)
	}
	if rec.count() != 1 {
		t.Errorf("requests = %d", rec.count())
	}

	down, _ := newServer(t, func(_ int, w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusBadGateway)
	})
	c, _ = testClient(down)
	healthy, err = c.Health(context.Background())
	if healthy || !errors.Is(err, ErrAPI) {
		t.Errorf("Health = %v, %v", healthy, err)
	}
}

func TestParseRetryAfter(t *testing.T) {
	if d := parseRetryAfter("2"); d != 2*time.Second {
		t.Errorf("2 -> %v", d)
	}
	if d := parseRetryAfter(""); d != 0 {
		t.Errorf("empty -> %v", d)
	}
	if d := parseRetryAfter("soon"); d != 0 {
		t.Errorf("soon -> %v", d)
	}
	future := time.Now().Add(10 * time.Second).UTC().Format(http.TimeFormat)
	if d := parseRetryAfter(future); d <= 0 || d > 11*time.Second {
		t.Errorf("date -> %v", d)
	}
}
