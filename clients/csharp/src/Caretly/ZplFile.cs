using System;
using System.IO;
using System.Text;

namespace Caretly
{
    /// <summary>One named ZPL file of a multi-file render.</summary>
    public sealed class ZplFile
    {
        /// <summary>Creates a file from ZPL text, which is sent as UTF-8.</summary>
        /// <param name="name">The file name, sent as the multipart filename.</param>
        /// <param name="zpl">The ZPL text.</param>
        public ZplFile(string name, string zpl)
            : this(name, Encoding.UTF8.GetBytes(zpl ?? throw new ArgumentNullException(nameof(zpl))))
        {
        }

        /// <summary>Creates a file from raw ZPL bytes, sent as is.</summary>
        /// <param name="name">The file name, sent as the multipart filename.</param>
        /// <param name="content">The ZPL bytes.</param>
        public ZplFile(string name, byte[] content)
        {
            if (string.IsNullOrEmpty(name))
            {
                throw new ArgumentException("A file name is required.", nameof(name));
            }

            Name = name;
            Content = content ?? throw new ArgumentNullException(nameof(content));
        }

        /// <summary>The file name.</summary>
        public string Name { get; }

        /// <summary>The ZPL bytes.</summary>
        public byte[] Content { get; }

        /// <summary>Reads a ZPL file from disk, named after the file.</summary>
        /// <param name="path">The path of the file.</param>
        /// <returns>The file.</returns>
        public static ZplFile FromPath(string path)
        {
            if (path == null)
            {
                throw new ArgumentNullException(nameof(path));
            }

            return new ZplFile(Path.GetFileName(path), File.ReadAllBytes(path));
        }
    }
}
