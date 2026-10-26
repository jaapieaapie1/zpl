using System.Text;

namespace Caretly.Tests;

/// <summary>A small independent multipart parser, to check the client's body is well formed.</summary>
internal sealed class MultipartFormDataContentReader
{
    public MultipartFormDataContentReader(byte[] body, string boundary)
    {
        var text = Encoding.UTF8.GetString(body);
        var delimiter = "--" + boundary;
        Assert(text.EndsWith(delimiter + "--\r\n"), "missing closing delimiter");
        var sections = text.Substring(0, text.Length - (delimiter.Length + 4)).Split(delimiter + "\r\n", StringSplitOptions.RemoveEmptyEntries);
        foreach (var section in sections)
        {
            var split = section.IndexOf("\r\n\r\n", StringComparison.Ordinal);
            Assert(split > 0, "part without headers");
            var headers = section.Substring(0, split).Split("\r\n");
            var disposition = headers.Single(h => h.StartsWith("Content-Disposition:", StringComparison.OrdinalIgnoreCase));
            Assert(disposition.Contains("name=\"file\""), "part not named file");
            var start = disposition.IndexOf("filename=\"", StringComparison.Ordinal) + "filename=\"".Length;
            var fileName = disposition.Substring(start, disposition.IndexOf('"', start) - start);
            Assert(section.EndsWith("\r\n"), "part without trailing CRLF");
            Parts.Add((fileName, section.Substring(split + 4, section.Length - split - 6)));
        }
    }

    public List<(string FileName, string Content)> Parts { get; } = new();

    private static void Assert(bool condition, string message)
    {
        if (!condition)
        {
            throw new FormatException(message);
        }
    }
}
