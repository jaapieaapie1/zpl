// Package caretly is a client for the Caretly API (https://caretly.dev),
// which renders ZPL labels to vector PDF or PNG.
//
// Create a client and render a label:
//
//	client := caretly.NewClient(caretly.WithAPIKey("zpl_test_..."))
//	res, err := client.Render(ctx, "^XA^FO50,50^A0N,50,50^FDHello^FS^XZ", nil)
//	if err != nil {
//		log.Fatal(err)
//	}
//	err = res.Save("label.pdf")
//
// Without WithAPIKey the key is read from the CARETLY_API_KEY environment
// variable. The key is optional for self-hosted servers (see WithBaseURL).
// Live keys (zpl_live_…) are billed per label; test keys (zpl_test_…) are
// free and watermark every label.
//
// # Options
//
// RenderOptions selects the format (FormatPDF or FormatPNG), the PNG page,
// the printer resolution (DPMM: 6, 8, 12 or 24), server timings, an
// idempotency key and gzip compression of the request body. Invalid options
// fail before any request with an error matching ErrInvalidOptions.
//
//	res, err := client.Render(ctx, zpl, &caretly.RenderOptions{
//		Format: caretly.FormatPNG,
//		Page:   2,
//		DPMM:   12,
//	})
//
// RenderFiles sends several ZPL files in one multipart request. Each file
// starts from a fresh printer state and pages follow upload order;
// RenderResult.FilePageCounts gives the pages of each file.
//
// # Errors
//
// A non-200 response is returned as an *APIError holding the status code,
// the server's message and the Retry-After wait. Match it with errors.Is
// against ErrBadRequest, ErrUnauthorized, ErrPaymentRequired,
// ErrPayloadTooLarge, ErrUnsupportedEncoding, ErrRateLimited, ErrOverloaded
// or ErrAPI (any API error), or with errors.As to read its fields:
//
//	var apiErr *caretly.APIError
//	if errors.As(err, &apiErr) {
//		log.Printf("status %d: %s", apiErr.StatusCode, apiErr.Message)
//	}
//
// Transport failures are a *TransportError matching ErrConnection or
// ErrTimeout. When ctx is cancelled the context's error is returned.
//
// # Retries
//
// A 429, a 503 and a connection error are retried up to WithMaxRetries times
// (default 2), waiting Retry-After when the server sends it and otherwise
// 0.5s, 1s, 2s… up to 8s. Timeouts and other statuses are not retried. The
// wait stops early when ctx is done. So that a retry is never billed twice,
// the client sends an Idempotency-Key, generated once per call when
// RenderOptions.IdempotencyKey is empty.
package caretly
