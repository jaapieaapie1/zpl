package caretly

import (
	"bytes"
	"compress/gzip"
	"context"
	"crypto/rand"
	"errors"
	"fmt"
	"io"
	"mime/multipart"
	"net/http"
	"net/url"
	"os"
	"strconv"
	"strings"
	"time"
)

// Format is the output format of a render.
type Format string

const (
	// FormatPDF is a vector PDF holding every page (the server default).
	FormatPDF Format = "pdf"
	// FormatPNG is a single page as a PNG image, chosen with RenderOptions.Page.
	FormatPNG Format = "png"
)

// RenderOptions tunes a render. The zero value, or a nil pointer, uses the
// server defaults: a PDF at 8 dots per millimetre.
type RenderOptions struct {
	// Format is FormatPDF or FormatPNG. Empty uses the server default (PDF).
	Format Format
	// Page is the page to show in a PNG, starting at 1 and counted across
	// all files. Zero uses the server default (1).
	Page int
	// DPMM is the printer resolution in dots per millimetre: 6, 8, 12 or 24.
	// Zero uses the server default (8).
	DPMM int
	// Timings asks the server for a Server-Timing header, exposed as
	// RenderResult.ServerTiming.
	Timings bool
	// IdempotencyKey is sent as the Idempotency-Key header (max 255
	// characters). A retry with the same key and content is billed once.
	// When empty and retries are enabled, the client generates one per call.
	IdempotencyKey string
	// Compress gzips the request body and sends Content-Encoding: gzip.
	Compress bool
}

// File is one ZPL file of a multipart render. Each file starts from a fresh
// printer state and its pages follow the previous file's.
type File struct {
	// Name is the filename sent with the part, for example "label.zpl".
	Name string
	// Content is the ZPL. Use []byte(s) for a string.
	Content []byte
}

// RateLimit is the rate limit state of the API key after a request.
type RateLimit struct {
	// Limit is the burst size: how many requests the key can make at once.
	Limit int
	// Remaining is how many requests the key can still make at once.
	Remaining int
}

// RenderResult is a rendered PDF or PNG with the metadata from its headers.
type RenderResult struct {
	// Content is the file bytes.
	Content []byte
	// ContentType is "application/pdf" or "image/png".
	ContentType string
	// PageCount is the total number of pages across the input.
	PageCount int
	// FilePageCounts holds the pages per file for a multipart render, in
	// upload order. It is empty for a single label.
	FilePageCounts []int
	// LabelsBilled is the number of labels billed (0 for a test key), or nil
	// when the server does not bill (no API keys).
	LabelsBilled *int
	// RateLimit is the key's rate limit state, or nil when not sent.
	RateLimit *RateLimit
	// ServerTiming is the raw Server-Timing header, or "" when not sent.
	ServerTiming string
}

// Save writes the rendered file to path, creating or truncating it.
func (r *RenderResult) Save(path string) error {
	return os.WriteFile(path, r.Content, 0o644)
}

// Render renders a single ZPL document, which may hold several labels
// (^XA…^XZ). opts may be nil.
func (c *Client) Render(ctx context.Context, zpl string, opts *RenderOptions) (*RenderResult, error) {
	return c.render(ctx, []byte(zpl), "text/plain; charset=utf-8", opts)
}

// RenderFiles renders several ZPL files in one multipart request. Pages
// follow upload order. opts may be nil.
func (c *Client) RenderFiles(ctx context.Context, files []File, opts *RenderOptions) (*RenderResult, error) {
	if len(files) == 0 {
		return nil, fmt.Errorf("%w: no files", ErrInvalidOptions)
	}
	var buf bytes.Buffer
	mw := multipart.NewWriter(&buf)
	for i, f := range files {
		name := f.Name
		if name == "" {
			name = fmt.Sprintf("file%d.zpl", i+1)
		}
		part, err := mw.CreateFormFile("file", name)
		if err != nil {
			return nil, err
		}
		if _, err := part.Write(f.Content); err != nil {
			return nil, err
		}
	}
	if err := mw.Close(); err != nil {
		return nil, err
	}
	return c.render(ctx, buf.Bytes(), mw.FormDataContentType(), opts)
}

// RenderPDF renders zpl to a PDF with the server defaults.
func (c *Client) RenderPDF(ctx context.Context, zpl string) (*RenderResult, error) {
	return c.Render(ctx, zpl, &RenderOptions{Format: FormatPDF})
}

// RenderPNG renders one page of zpl, starting at 1, to a PNG.
func (c *Client) RenderPNG(ctx context.Context, zpl string, page int) (*RenderResult, error) {
	return c.Render(ctx, zpl, &RenderOptions{Format: FormatPNG, Page: page})
}

func (o *RenderOptions) validate() error {
	switch o.Format {
	case "", FormatPDF, FormatPNG:
	default:
		return fmt.Errorf("%w: format %q is not pdf or png", ErrInvalidOptions, o.Format)
	}
	if o.Page < 0 {
		return fmt.Errorf("%w: page must be 1 or more, got %d", ErrInvalidOptions, o.Page)
	}
	switch o.DPMM {
	case 0, 6, 8, 12, 24:
	default:
		return fmt.Errorf("%w: dpmm must be 6, 8, 12 or 24, got %d", ErrInvalidOptions, o.DPMM)
	}
	if len(o.IdempotencyKey) > 255 {
		return fmt.Errorf("%w: idempotency key is longer than 255 characters", ErrInvalidOptions)
	}
	return nil
}

func (o *RenderOptions) query() string {
	q := url.Values{}
	if o.Format != "" {
		q.Set("format", string(o.Format))
	}
	if o.Page > 0 {
		q.Set("page", strconv.Itoa(o.Page))
	}
	if o.DPMM > 0 {
		q.Set("dpmm", strconv.Itoa(o.DPMM))
	}
	if o.Timings {
		q.Set("timings", "1")
	}
	return q.Encode()
}

func (c *Client) render(ctx context.Context, body []byte, contentType string, opts *RenderOptions) (*RenderResult, error) {
	if opts == nil {
		opts = &RenderOptions{}
	}
	if err := opts.validate(); err != nil {
		return nil, err
	}
	endpoint := c.baseURL + "/render"
	if q := opts.query(); q != "" {
		endpoint += "?" + q
	}
	if opts.Compress {
		var gz bytes.Buffer
		zw := gzip.NewWriter(&gz)
		if _, err := zw.Write(body); err != nil {
			return nil, err
		}
		if err := zw.Close(); err != nil {
			return nil, err
		}
		body = gz.Bytes()
	}
	idempotencyKey := opts.IdempotencyKey
	if idempotencyKey == "" && c.maxRetries > 0 {
		key, err := newUUID()
		if err != nil {
			return nil, err
		}
		idempotencyKey = key
	}

	for attempt := 0; ; attempt++ {
		res, err := c.renderOnce(ctx, endpoint, body, contentType, opts.Compress, idempotencyKey)
		if err == nil {
			return res, nil
		}
		wait, retryable := retryDelay(err, attempt)
		if !retryable || attempt >= c.maxRetries {
			return nil, err
		}
		if serr := c.sleep(ctx, wait); serr != nil {
			return nil, serr
		}
	}
}

func (c *Client) renderOnce(ctx context.Context, endpoint string, body []byte, contentType string, gzipped bool, idempotencyKey string) (*RenderResult, error) {
	actx, cancel := c.attemptContext(ctx)
	defer cancel()
	req, err := http.NewRequestWithContext(actx, http.MethodPost, endpoint, bytes.NewReader(body))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", contentType)
	req.Header.Set("User-Agent", userAgent)
	if c.apiKey != "" {
		req.Header.Set("Authorization", "Bearer "+c.apiKey)
	}
	if idempotencyKey != "" {
		req.Header.Set("Idempotency-Key", idempotencyKey)
	}
	if gzipped {
		req.Header.Set("Content-Encoding", "gzip")
	}
	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, transportError(ctx, actx, err)
	}
	defer resp.Body.Close()
	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, transportError(ctx, actx, err)
	}
	if resp.StatusCode != http.StatusOK {
		return nil, newAPIError(resp, data)
	}
	return newRenderResult(resp.Header, data), nil
}

func newRenderResult(h http.Header, data []byte) *RenderResult {
	r := &RenderResult{
		Content:        data,
		ContentType:    h.Get("Content-Type"),
		FilePageCounts: []int{},
		ServerTiming:   h.Get("Server-Timing"),
	}
	if n, ok := headerInt(h, "X-Page-Count"); ok {
		r.PageCount = n
	}
	if v := h.Get("X-File-Page-Counts"); v != "" {
		for _, s := range strings.Split(v, ",") {
			if n, err := strconv.Atoi(strings.TrimSpace(s)); err == nil {
				r.FilePageCounts = append(r.FilePageCounts, n)
			}
		}
	}
	if n, ok := headerInt(h, "X-Labels-Billed"); ok {
		r.LabelsBilled = &n
	}
	limit, okL := headerInt(h, "RateLimit-Limit")
	remaining, okR := headerInt(h, "RateLimit-Remaining")
	if okL || okR {
		r.RateLimit = &RateLimit{Limit: limit, Remaining: remaining}
	}
	return r
}

func headerInt(h http.Header, name string) (int, bool) {
	v := strings.TrimSpace(h.Get(name))
	if v == "" {
		return 0, false
	}
	n, err := strconv.Atoi(v)
	return n, err == nil
}

const (
	baseBackoff = 500 * time.Millisecond
	maxBackoff  = 8 * time.Second
)

// retryDelay says whether err may be retried and how long to wait first.
func retryDelay(err error, attempt int) (time.Duration, bool) {
	var apiErr *APIError
	if errors.As(err, &apiErr) {
		if apiErr.StatusCode != http.StatusTooManyRequests && apiErr.StatusCode != http.StatusServiceUnavailable {
			return 0, false
		}
		if apiErr.RetryAfter > 0 {
			return apiErr.RetryAfter, true
		}
		return backoff(attempt), true
	}
	if errors.Is(err, ErrConnection) {
		return backoff(attempt), true
	}
	return 0, false
}

// backoff is 0.5s, 1s, 2s, … capped at 8s.
func backoff(attempt int) time.Duration {
	d := baseBackoff
	for i := 0; i < attempt && d < maxBackoff; i++ {
		d *= 2
	}
	if d > maxBackoff {
		d = maxBackoff
	}
	return d
}

// newUUID returns a random (version 4) UUID.
func newUUID() (string, error) {
	var b [16]byte
	if _, err := rand.Read(b[:]); err != nil {
		return "", err
	}
	b[6] = b[6]&0x0f | 0x40
	b[8] = b[8]&0x3f | 0x80
	return fmt.Sprintf("%x-%x-%x-%x-%x", b[0:4], b[4:6], b[6:8], b[8:10], b[10:16]), nil
}
