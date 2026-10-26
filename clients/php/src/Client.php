<?php

declare(strict_types=1);

namespace Caretly;

use Caretly\Exception\ApiException;
use Caretly\Exception\CaretlyException;
use Caretly\Exception\OverloadedException;
use Caretly\Exception\RateLimitedException;
use Caretly\Exception\TimeoutException;
use Caretly\Exception\TransportException;
use Closure;
use Http\Discovery\Psr17FactoryDiscovery;
use Http\Discovery\Psr18ClientDiscovery;
use InvalidArgumentException;
use Psr\Http\Client\ClientExceptionInterface;
use Psr\Http\Client\ClientInterface;
use Psr\Http\Client\NetworkExceptionInterface;
use Psr\Http\Message\RequestFactoryInterface;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\StreamFactoryInterface;

/**
 * Client for the Caretly API: render ZPL labels to PDF or PNG.
 */
final class Client
{
    public const VERSION = '0.1.0';
    public const DEFAULT_BASE_URL = 'https://caretly.dev';

    private const MAX_BACKOFF_SECONDS = 8.0;

    private readonly ?string $apiKey;
    private readonly string $baseUrl;
    private readonly ClientInterface $httpClient;
    private readonly RequestFactoryInterface $requestFactory;
    private readonly StreamFactoryInterface $streamFactory;
    /** @var Closure(float): void */
    private readonly Closure $sleep;

    /**
     * @param string|null                  $apiKey         API key; falls back to the CARETLY_API_KEY environment
     *                                                     variable. Optional for self-hosted servers.
     * @param string                       $baseUrl        Base URL of the API.
     * @param int                          $maxRetries     Retries for 429, 503 and connection failures;
     *                                                     timeouts are never retried.
     * @param float                        $timeout        Request timeout in seconds. Applied when the client
     *                                                     creates a Guzzle or Symfony HTTP client itself; an
     *                                                     injected $httpClient keeps its own timeout.
     * @param ClientInterface|null         $httpClient     Any PSR-18 client; discovered when null.
     * @param RequestFactoryInterface|null $requestFactory PSR-17 request factory; discovered when null.
     * @param StreamFactoryInterface|null  $streamFactory  PSR-17 stream factory; discovered when null.
     * @param (Closure(float): void)|null  $sleep          Called with the seconds to wait before a retry;
     *                                                     defaults to usleep(). Override it in tests.
     */
    public function __construct(
        ?string $apiKey = null,
        string $baseUrl = self::DEFAULT_BASE_URL,
        private readonly int $maxRetries = 2,
        float $timeout = 30.0,
        ?ClientInterface $httpClient = null,
        ?RequestFactoryInterface $requestFactory = null,
        ?StreamFactoryInterface $streamFactory = null,
        ?Closure $sleep = null,
    ) {
        if ($maxRetries < 0) {
            throw new InvalidArgumentException('maxRetries must be 0 or more');
        }
        if ($timeout <= 0) {
            throw new InvalidArgumentException('timeout must be more than 0 seconds');
        }
        if ($apiKey === null) {
            $env = getenv('CARETLY_API_KEY');
            $apiKey = is_string($env) ? $env : null;
        }
        $this->apiKey = $apiKey === '' ? null : $apiKey;
        $this->baseUrl = rtrim($baseUrl, '/');
        $this->httpClient = $httpClient ?? self::defaultHttpClient($timeout);
        $this->requestFactory = $requestFactory ?? Psr17FactoryDiscovery::findRequestFactory();
        $this->streamFactory = $streamFactory ?? Psr17FactoryDiscovery::findStreamFactory();
        $this->sleep = $sleep ?? static function (float $seconds): void {
            usleep((int) round($seconds * 1_000_000));
        };
    }

    /**
     * Render one ZPL document (which may hold several labels).
     *
     * @throws ApiException       when the API answers with an error status.
     * @throws TransportException when no response arrives.
     */
    public function render(string $zpl, ?RenderOptions $options = null): RenderResult
    {
        $options ??= new RenderOptions();

        return $this->send($options, 'text/plain; charset=utf-8', $zpl);
    }

    /**
     * Render several ZPL files in one request. Each file starts from a fresh printer state,
     * and pages follow the order of the array.
     *
     * @param array<string, string> $files File name => ZPL contents.
     *
     * @throws ApiException       when the API answers with an error status.
     * @throws TransportException when no response arrives.
     */
    public function renderFiles(array $files, ?RenderOptions $options = null): RenderResult
    {
        if ($files === []) {
            throw new InvalidArgumentException('renderFiles needs at least one file');
        }
        $options ??= new RenderOptions();
        $boundary = 'caretly-' . bin2hex(random_bytes(16));
        $body = '';
        foreach ($files as $name => $contents) {
            $filename = str_replace(['"', "\r", "\n", '\\'], ['%22', '', '', '\\\\'], (string) $name);
            $body .= "--{$boundary}\r\n"
                . "Content-Disposition: form-data; name=\"file\"; filename=\"{$filename}\"\r\n"
                . "Content-Type: text/plain; charset=utf-8\r\n\r\n"
                . $contents . "\r\n";
        }
        $body .= "--{$boundary}--\r\n";

        return $this->send($options, "multipart/form-data; boundary={$boundary}", $body);
    }

    /**
     * Render to PDF. Shorthand for render() with Format::Pdf.
     */
    public function renderPdf(string $zpl, ?int $dpmm = null): RenderResult
    {
        return $this->render($zpl, new RenderOptions(format: Format::Pdf, dpmm: $dpmm));
    }

    /**
     * Render one page to PNG. Shorthand for render() with Format::Png.
     */
    public function renderPng(string $zpl, int $page = 1, ?int $dpmm = null): RenderResult
    {
        return $this->render($zpl, new RenderOptions(format: Format::Png, page: $page, dpmm: $dpmm));
    }

    /**
     * Whether the server answers `GET /health` with `ok`. Never throws; a failed request is false.
     */
    public function health(): bool
    {
        $request = $this->requestFactory->createRequest('GET', $this->baseUrl . '/health')
            ->withHeader('User-Agent', 'caretly-php/' . self::VERSION);
        try {
            $response = $this->httpClient->sendRequest($request);
        } catch (ClientExceptionInterface) {
            return false;
        }

        return $response->getStatusCode() === 200 && trim((string) $response->getBody()) === 'ok';
    }

    private function send(RenderOptions $options, string $contentType, string $body): RenderResult
    {
        $url = $this->baseUrl . '/render';
        $query = $options->toQuery();
        if ($query !== []) {
            $url .= '?' . http_build_query($query, '', '&', PHP_QUERY_RFC3986);
        }

        $headers = [
            'Content-Type' => $contentType,
            'Accept' => 'application/pdf, image/png',
            'User-Agent' => 'caretly-php/' . self::VERSION,
        ];
        if ($this->apiKey !== null) {
            $headers['Authorization'] = 'Bearer ' . $this->apiKey;
        }
        $idempotencyKey = $options->idempotencyKey ?? ($this->maxRetries > 0 ? self::uuid4() : null);
        if ($idempotencyKey !== null) {
            $headers['Idempotency-Key'] = $idempotencyKey;
        }
        if ($options->compress) {
            $compressed = gzencode($body);
            if ($compressed === false) {
                throw new CaretlyException('could not gzip the request body');
            }
            $body = $compressed;
            $headers['Content-Encoding'] = 'gzip';
        }

        for ($attempt = 0;; $attempt++) {
            // A fresh request (and body stream) per attempt, so a retry never sends a consumed stream.
            $request = $this->requestFactory->createRequest('POST', $url)
                ->withBody($this->streamFactory->createStream($body));
            foreach ($headers as $name => $value) {
                $request = $request->withHeader($name, $value);
            }

            try {
                $response = $this->sendRequest($request);
            } catch (TransportException $e) {
                // A timeout may mean the server is still rendering; fail fast instead of piling on load.
                if ($e instanceof TimeoutException || $attempt >= $this->maxRetries) {
                    throw $e;
                }
                ($this->sleep)(self::backoff($attempt));
                continue;
            }

            if ($response->getStatusCode() === 200) {
                return self::result($response);
            }

            $error = ApiException::fromStatus(
                $response->getStatusCode(),
                trim((string) $response->getBody()),
                self::retryAfter($response),
            );
            $retryable = $error instanceof RateLimitedException || $error instanceof OverloadedException;
            if (!$retryable || $attempt >= $this->maxRetries) {
                throw $error;
            }
            ($this->sleep)($error->retryAfter !== null ? (float) $error->retryAfter : self::backoff($attempt));
        }
    }

    private function sendRequest(RequestInterface $request): ResponseInterface
    {
        try {
            return $this->httpClient->sendRequest($request);
        } catch (NetworkExceptionInterface $e) {
            $message = $e->getMessage();
            if (preg_match('/time[d ]?\s*out|cURL error 28/i', $message) === 1) {
                throw new TimeoutException($message, 0, $e);
            }
            throw new TransportException($message, 0, $e);
        } catch (ClientExceptionInterface $e) {
            // Not a network failure (e.g. a request the HTTP client refuses): retrying will not help.
            throw new CaretlyException($e->getMessage(), 0, $e);
        }
    }

    private static function result(ResponseInterface $response): RenderResult
    {
        $filePageCounts = [];
        $header = $response->getHeaderLine('X-File-Page-Counts');
        if ($header !== '') {
            foreach (explode(',', $header) as $count) {
                $filePageCounts[] = (int) trim($count);
            }
        }
        $limit = self::intHeader($response, 'RateLimit-Limit');
        $remaining = self::intHeader($response, 'RateLimit-Remaining');
        $serverTiming = $response->getHeaderLine('Server-Timing');

        return new RenderResult(
            content: (string) $response->getBody(),
            contentType: $response->getHeaderLine('Content-Type'),
            pageCount: self::intHeader($response, 'X-Page-Count') ?? 0,
            filePageCounts: $filePageCounts,
            labelsBilled: self::intHeader($response, 'X-Labels-Billed'),
            rateLimit: $limit === null && $remaining === null ? null : new RateLimit($limit, $remaining),
            serverTiming: $serverTiming === '' ? null : $serverTiming,
        );
    }

    private static function intHeader(ResponseInterface $response, string $name): ?int
    {
        $value = trim($response->getHeaderLine($name));

        return ctype_digit($value) ? (int) $value : null;
    }

    /**
     * `Retry-After` in whole seconds: either delay-seconds or an HTTP date.
     */
    private static function retryAfter(ResponseInterface $response): ?int
    {
        $value = trim($response->getHeaderLine('Retry-After'));
        if ($value === '') {
            return null;
        }
        if (ctype_digit($value)) {
            return (int) $value;
        }
        $time = strtotime($value);

        return $time === false ? null : max(0, $time - time());
    }

    /** 0.5 s, 1 s, 2 s, … capped at 8 s. */
    private static function backoff(int $attempt): float
    {
        return min(self::MAX_BACKOFF_SECONDS, 0.5 * 2 ** $attempt);
    }

    private static function uuid4(): string
    {
        $bytes = random_bytes(16);
        $bytes[6] = chr((ord($bytes[6]) & 0x0f) | 0x40);
        $bytes[8] = chr((ord($bytes[8]) & 0x3f) | 0x80);

        return vsprintf('%s%s-%s-%s-%s-%s%s%s', str_split(bin2hex($bytes), 4));
    }

    private static function defaultHttpClient(float $timeout): ClientInterface
    {
        if (class_exists(\GuzzleHttp\Client::class)) {
            return new \GuzzleHttp\Client(['timeout' => $timeout, 'connect_timeout' => min($timeout, 10.0)]);
        }
        if (class_exists(\Symfony\Component\HttpClient\Psr18Client::class)) {
            return new \Symfony\Component\HttpClient\Psr18Client(
                \Symfony\Component\HttpClient\HttpClient::create(['timeout' => $timeout, 'max_duration' => $timeout]),
            );
        }

        return Psr18ClientDiscovery::find();
    }
}
