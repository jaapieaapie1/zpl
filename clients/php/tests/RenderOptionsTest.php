<?php

declare(strict_types=1);

namespace Caretly\Tests;

use Caretly\Format;
use Caretly\RenderOptions;
use Caretly\RenderResult;
use InvalidArgumentException;
use PHPUnit\Framework\TestCase;

final class RenderOptionsTest extends TestCase
{
    public function testQueryLeavesOutUnsetOptions(): void
    {
        self::assertSame([], (new RenderOptions())->toQuery());
        self::assertSame(
            ['format' => 'png', 'page' => '2', 'dpmm' => '24', 'timings' => '1'],
            (new RenderOptions(format: Format::Png, page: 2, dpmm: 24, timings: true))->toQuery(),
        );
    }

    public function testPageMustBePositive(): void
    {
        $this->expectException(InvalidArgumentException::class);
        new RenderOptions(page: 0);
    }

    public function testDpmmMustBeSupported(): void
    {
        $this->expectException(InvalidArgumentException::class);
        new RenderOptions(dpmm: 10);
    }

    public function testIdempotencyKeyLength(): void
    {
        new RenderOptions(idempotencyKey: str_repeat('k', 255));
        $this->expectException(InvalidArgumentException::class);
        new RenderOptions(idempotencyKey: str_repeat('k', 256));
    }

    public function testFormatContentType(): void
    {
        self::assertSame('application/pdf', Format::Pdf->contentType());
        self::assertSame('image/png', Format::from('png')->contentType());
    }

    public function testSaveTo(): void
    {
        $path = tempnam(sys_get_temp_dir(), 'caretly');
        self::assertIsString($path);
        try {
            (new RenderResult('%PDF-bytes', 'application/pdf', 1))->saveTo($path);
            self::assertSame('%PDF-bytes', file_get_contents($path));
        } finally {
            @unlink($path);
        }
    }
}
