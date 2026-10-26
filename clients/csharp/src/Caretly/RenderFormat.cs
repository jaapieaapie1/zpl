namespace Caretly
{
    /// <summary>The output format of a render.</summary>
    public enum RenderFormat
    {
        /// <summary>A vector PDF holding every page. The server default.</summary>
        Pdf,

        /// <summary>A PNG image of one page, chosen with <see cref="RenderOptions.Page"/>.</summary>
        Png,
    }
}
