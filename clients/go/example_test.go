package caretly_test

import (
	"context"
	"errors"
	"fmt"
	"log"
	"net/http"
	"net/http/httptest"

	caretly "github.com/jaapieaapie1/zpl/clients/go"
)

// fakeServer stands in for the Caretly API so the examples run offline.
func fakeServer() *httptest.Server {
	mux := http.NewServeMux()
	mux.HandleFunc("/render", func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("Authorization") != "Bearer zpl_test_example" {
			w.WriteHeader(http.StatusUnauthorized)
			fmt.Fprint(w, "The API key is not valid.")
			return
		}
		if r.URL.Query().Get("format") == "png" {
			w.Header().Set("Content-Type", "image/png")
		} else {
			w.Header().Set("Content-Type", "application/pdf")
		}
		w.Header().Set("X-Page-Count", "1")
		w.Header().Set("X-Labels-Billed", "0")
		fmt.Fprint(w, "%PDF-1.7 ...")
	})
	mux.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "ok")
	})
	return httptest.NewServer(mux)
}

func ExampleClient_Render() {
	srv := fakeServer()
	defer srv.Close()

	client := caretly.NewClient(
		caretly.WithAPIKey("zpl_test_example"),
		caretly.WithBaseURL(srv.URL), // omit to use https://caretly.dev
	)
	res, err := client.Render(context.Background(), "^XA^FO50,50^A0N,50,50^FDHello^FS^XZ", &caretly.RenderOptions{
		Format: caretly.FormatPNG,
		Page:   1,
		DPMM:   8,
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(res.ContentType, res.PageCount, *res.LabelsBilled)
	// Output: image/png 1 0
}

func ExampleClient_RenderFiles() {
	srv := fakeServer()
	defer srv.Close()

	client := caretly.NewClient(caretly.WithAPIKey("zpl_test_example"), caretly.WithBaseURL(srv.URL))
	res, err := client.RenderFiles(context.Background(), []caretly.File{
		{Name: "a.zpl", Content: []byte("^XA^FDfirst^FS^XZ")},
		{Name: "b.zpl", Content: []byte("^XA^FDsecond^FS^XZ")},
	}, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(res.ContentType)
	// Output: application/pdf
}

func ExampleAPIError() {
	srv := fakeServer()
	defer srv.Close()

	client := caretly.NewClient(caretly.WithAPIKey("zpl_live_wrong"), caretly.WithBaseURL(srv.URL))
	_, err := client.Render(context.Background(), "^XA^XZ", nil)

	var apiErr *caretly.APIError
	if errors.As(err, &apiErr) {
		fmt.Println(apiErr.StatusCode, apiErr.Message)
	}
	fmt.Println(errors.Is(err, caretly.ErrUnauthorized))
	// Output:
	// 401 The API key is not valid.
	// true
}

func ExampleClient_Health() {
	srv := fakeServer()
	defer srv.Close()

	client := caretly.NewClient(caretly.WithBaseURL(srv.URL))
	ok, err := client.Health(context.Background())
	fmt.Println(ok, err)
	// Output: true <nil>
}
