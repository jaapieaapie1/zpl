<?php

declare(strict_types=1);

namespace Caretly\Tests;

use Caretly\Client;
use Caretly\Exception\ApiException;
use Caretly\Exception\AuthenticationException;
use Caretly\Exception\BadRequestException;
use Caretly\Exception\OverloadedException;
use Caretly\Exception\PayloadTooLargeException;
use Caretly\Exception\PaymentRequiredException;
use Caretly\Exception\RateLimitedException;
use Caretly\Exception\TimeoutException;
use Caretly\Exception\TransportException;
use Caretly\Exception\UnsupportedEncodingException;
use Caretly\Format;
use Caretly\RenderOptions;
use GuzzleHttp\Psr7\HttpFactory;
use GuzzleHttp\Psr7\Response;
use Http\Client\Exception\NetworkException;
use Http\Mock\Client as MockClient;
use InvalidArgumentException;
use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;
use Psr\Http\Message\RequestInterface;

final class ClientTest extends TestCase
{
    private MockClient $http;
    /** @var list<float> */
    private array $sleeps = [];

    protected function setUp(): void
    {
        $this->http = new MockClient(new HttpFactory());
        $this->sleeps = [];
    }

    private function client(?string $apiKey = 'zpl_test_abc', int $maxRetries = 2): Client
    {
        $factory = new HttpFactory();

        return new Client(
            apiKey: $apiKey,
            baseUrl: 'http://localhost:8080/',
            maxRetries: $maxRetries,
            httpClient: $this->http,
            requestFactory: $factory,
            streamFactory: $factory,
            sleep: function (float $seconds): void {
                $this->sleeps[] = $seconds;
            },
        );
    }

    private function lastRequest(): RequestInterface
    {
        $request = $this->http->getLastRequest();
        self::assertInstanceOf(RequestInterface::class, $request);

        return $request;
    }

    public function testRenderSendsZplAndParsesHeaders(): void
    {
        $this->http->addResponse(new Response(200, [
            'Content-Type' => 'application/pdf',
            'X-Page-Count' => '3',
            'X-Labels-Billed' => '3',
            'RateLimit-Limit' => '200',
            'RateLimit-Remaining' => '199',
            'Server-Timing' => 'read;dur=0.1, parse;dur=0.2',
        ], '%PDF-1.7'));

        $result = $this->client()->render('^XA^FDHi^FS^XZ', new RenderOptions(
            format: Format::Pdf,
            dpmm: 12,
            timings: true,
            idempotencyKey: 'order-42',
        ));

        self::assertSame('%PDF-1.7', $result->content);
        self::assertSame('application/pdf', $result->contentType);
        self::assertSame(3, $result->pageCount);
        self::assertSame([], $result->filePageCounts);
        self::assertSame(3, $result->labelsBilled);
        self::assertNotNull($result->rateLimit);
        self::assertSame(200, $result->rateLimit->limit);
        self::assertSame(199, $result->rateLimit->remaining);
        self::assertSame('read;dur=0.1, parse;dur=0.2', $result->serverTiming);

        $request = $this->lastRequest();
        self::assertSame('POST', $request->getMethod());
        self::assertSame('http://localhost:8080/render?format=pdf&dpmm=12&timings=1', (string) $request->getUri());
        self::assertSame('Bearer zpl_test_abc', $request->getHeaderLine('Authorization'));
        self::assertSame('text/plain; charset=utf-8', $request->getHeaderLine('Content-Type'));
        self::assertSame('caretly-php/' . Client::VERSION, $request->getHeaderLine('User-Agent'));
        self::assertSame('order-42', $request->getHeaderLine('Idempotency-Key'));
        self::assertFalse($request->hasHeader('Content-Encoding'));
        self::assertSame('^XA^FDHi^FS^XZ', (string) $request->getBody());
    }

    public function testDefaultsOmitQueryAndOptionalHeaders(): void
    {
        $this->http->addResponse(new Response(200, ['Content-Type' => 'application/pdf', 'X-Page-Count' => '1'], 'x'));

        $result = $this->client(apiKey: '', maxRetries: 0)->render('^XA^XZ');

        $request = $this->lastRequest();
        self::assertSame('http://localhost:8080/render', (string) $request->getUri());
        self::assertFalse($request->hasHeader('Authorization'));
        self::assertFalse($request->hasHeader('Idempotency-Key'));
        self::assertNull($result->labelsBilled);
        self::assertNull($result->rateLimit);
        self::assertNull($result->serverTiming);
    }

    public function testApiKeyFallsBackToEnvironment(): void
    {
        putenv('CARETLY_API_KEY=zpl_live_env');
        try {
            $this->http->addResponse(new Response(200, [], ''));
            $this->client(apiKey: null)->render('^XA^XZ');
            self::assertSame('Bearer zpl_live_env', $this->lastRequest()->getHeaderLine('Authorization'));
        } finally {
            putenv('CARETLY_API_KEY');
        }
    }

    public function testRenderPngShorthand(): void
    {
        $this->http->addResponse(new Response(200, ['Content-Type' => 'image/png', 'X-Page-Count' => '4'], 'png'));

        $result = $this->client()->renderPng('^XA^XZ', page: 2);

        self::assertSame('image/png', $result->contentType);
        self::assertSame('http://localhost:8080/render?format=png&page=2', (string) $this->lastRequest()->getUri());
    }

    public function testRenderFilesSendsMultipart(): void
    {
        $this->http->addResponse(new Response(200, [
            'Content-Type' => 'application/pdf',
            'X-Page-Count' => '3',
            'X-File-Page-Counts' => '2, 1',
        ], '%PDF'));

        $result = $this->client()->renderFiles(['a.zpl' => '^XA^FDa^FS^XZ^XA^XZ', 'b.zpl' => '^XA^FDb^FS^XZ']);

        self::assertSame([2, 1], $result->filePageCounts);
        $request = $this->lastRequest();
        $contentType = $request->getHeaderLine('Content-Type');
        self::assertMatchesRegularExpression('/^multipart\/form-data; boundary=(.+)$/', $contentType);
        $boundary = substr($contentType, strlen('multipart/form-data; boundary='));
        $expected = "--{$boundary}\r\n"
            . "Content-Disposition: form-data; name=\"file\"; filename=\"a.zpl\"\r\n"
            . "Content-Type: text/plain; charset=utf-8\r\n\r\n"
            . "^XA^FDa^FS^XZ^XA^XZ\r\n"
            . "--{$boundary}\r\n"
            . "Content-Disposition: form-data; name=\"file\"; filename=\"b.zpl\"\r\n"
            . "Content-Type: text/plain; charset=utf-8\r\n\r\n"
            . "^XA^FDb^FS^XZ\r\n"
            . "--{$boundary}--\r\n";
        self::assertSame($expected, (string) $request->getBody());
    }

    public function testRenderFilesNeedsAFile(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->client()->renderFiles([]);
    }

    public function testCompressGzipsTheBody(): void
    {
        $this->http->addResponse(new Response(200, [], ''));
        $zpl = str_repeat('^XA^FDcompress me^FS^XZ', 50);

        $this->client()->render($zpl, new RenderOptions(compress: true));

        $request = $this->lastRequest();
        self::assertSame('gzip', $request->getHeaderLine('Content-Encoding'));
        $body = (string) $request->getBody();
        self::assertLessThan(strlen($zpl), strlen($body));
        self::assertSame($zpl, gzdecode($body));
    }

    /**
     * @return iterable<string, array{int, class-string<ApiException>}>
     */
    public static function errorStatuses(): iterable
    {
        yield '400' => [400, BadRequestException::class];
        yield '401' => [401, AuthenticationException::class];
        yield '402' => [402, PaymentRequiredException::class];
        yield '413' => [413, PayloadTooLargeException::class];
        yield '415' => [415, UnsupportedEncodingException::class];
        yield '429' => [429, RateLimitedException::class];
        yield '503' => [503, OverloadedException::class];
        yield '500' => [500, ApiException::class];
        yield '404' => [404, ApiException::class];
    }

    /**
     * @param class-string<ApiException> $class
     */
    #[DataProvider('errorStatuses')]
    public function testErrorStatusesMapToExceptions(int $status, string $class): void
    {
        $this->http->addResponse(new Response($status, ['Retry-After' => '7'], "message for {$status}\n"));

        try {
            $this->client(maxRetries: 0)->render('^XA^XZ');
            self::fail('expected an exception');
        } catch (ApiException $e) {
            self::assertSame($class, $e::class);
            self::assertSame($status, $e->status);
            self::assertSame("message for {$status}", $e->getMessage());
            self::assertSame(7, $e->retryAfter);
        }
        self::assertSame([], $this->sleeps);
    }

    public function testRetriesRateLimitHonoringRetryAfter(): void
    {
        $this->http->addResponse(new Response(429, ['Retry-After' => '3'], 'slow down'));
        $this->http->addResponse(new Response(200, ['X-Page-Count' => '1'], 'ok'));

        $result = $this->client()->render('^XA^XZ');

        self::assertSame('ok', $result->content);
        self::assertSame([3.0], $this->sleeps);
        $requests = $this->http->getRequests();
        self::assertCount(2, $requests);
        $key = $requests[0]->getHeaderLine('Idempotency-Key');
        self::assertMatchesRegularExpression('/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/', $key);
        self::assertSame($key, $requests[1]->getHeaderLine('Idempotency-Key'), 'same key on every attempt');
        self::assertSame('^XA^XZ', (string) $requests[1]->getBody(), 'retry sends the full body again');
    }

    public function testRetriesOverloadedWithBackoffThenGivesUp(): void
    {
        for ($i = 0; $i < 3; $i++) {
            $this->http->addResponse(new Response(503, [], 'busy'));
        }

        try {
            $this->client()->render('^XA^XZ');
            self::fail('expected an exception');
        } catch (OverloadedException $e) {
            self::assertSame(503, $e->status);
            self::assertNull($e->retryAfter);
        }
        self::assertSame([0.5, 1.0], $this->sleeps);
        self::assertCount(3, $this->http->getRequests());
    }

    public function testDoesNotRetryOtherStatuses(): void
    {
        $this->http->addResponse(new Response(500, [], 'boom'));

        try {
            $this->client()->render('^XA^XZ');
            self::fail('expected an exception');
        } catch (ApiException $e) {
            self::assertSame(500, $e->status);
        }
        self::assertCount(1, $this->http->getRequests());
        self::assertSame([], $this->sleeps);
    }

    public function testRetriesConnectionErrors(): void
    {
        $request = (new HttpFactory())->createRequest('POST', 'http://localhost:8080/render');
        $this->http->addException(new NetworkException('Connection refused', $request));
        $this->http->addResponse(new Response(200, [], 'ok'));

        $result = $this->client()->render('^XA^XZ');

        self::assertSame('ok', $result->content);
        self::assertSame([0.5], $this->sleeps);
    }

    public function testTransportErrorsAfterRetries(): void
    {
        $request = (new HttpFactory())->createRequest('POST', 'http://localhost:8080/render');
        $this->http->setDefaultException(new NetworkException('Connection refused', $request));

        $this->expectException(TransportException::class);
        $this->client(maxRetries: 1)->render('^XA^XZ');
    }

    public function testTimeoutsAreReported(): void
    {
        $request = (new HttpFactory())->createRequest('POST', 'http://localhost:8080/render');
        $this->http->setDefaultException(new NetworkException('cURL error 28: Operation timed out', $request));

        $this->expectException(TimeoutException::class);
        $this->client(maxRetries: 0)->render('^XA^XZ');
    }

    public function testTimeoutsAreNotRetried(): void
    {
        $request = (new HttpFactory())->createRequest('POST', 'http://localhost:8080/render');
        $this->http->addException(new NetworkException('cURL error 28: Operation timed out', $request));
        $this->http->addResponse(new Response(200, [], 'ok'));

        try {
            $this->client(maxRetries: 2)->render('^XA^XZ');
            self::fail('expected TimeoutException');
        } catch (TimeoutException) {
        }

        self::assertCount(1, $this->http->getRequests());
        self::assertSame([], $this->sleeps);
    }

    public function testHealth(): void
    {
        $this->http->addResponse(new Response(200, [], "ok"));
        self::assertTrue($this->client()->health());
        self::assertSame('http://localhost:8080/health', (string) $this->lastRequest()->getUri());
        self::assertSame('GET', $this->lastRequest()->getMethod());

        $this->http->addResponse(new Response(500, [], 'no'));
        self::assertFalse($this->client()->health());

        $request = (new HttpFactory())->createRequest('GET', 'http://localhost:8080/health');
        $this->http->addException(new NetworkException('down', $request));
        self::assertFalse($this->client()->health());
    }

    public function testDefaultHttpClientIsDiscovered(): void
    {
        $this->expectNotToPerformAssertions();
        new Client(apiKey: 'k');
    }
}
