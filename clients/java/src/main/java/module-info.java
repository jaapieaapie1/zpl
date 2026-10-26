/**
 * Client for the Caretly API: render ZPL labels to PDF or PNG.
 *
 * <p>Start with {@link dev.caretly.CaretlyClient}.
 */
module dev.caretly {
    requires transitive java.net.http;

    exports dev.caretly;
}
