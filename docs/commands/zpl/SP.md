# ^SP




ZPL Commands


The `^SP` command allows a label to start printing at a specified point before the entire label has been
completely formatted. On extremely complex labels, this command can increase the overall throughput of
the print.


**Start Print**

The command works as follows: Specify the dot row at which the `^SP` command is to begin. This creates
a label segment. Once the `^SP` command is processed all information in that segment prints. During the
printing process, all of the commands after the `^SP` continue to be received and processed by the printer.

If the segment after the `^SP` command (or the remainder of the label) is ready for printing, and media
motion does not stop. If the next segment is not ready, the printer stops mid-label and waits for the next
segment to be completed. Precise positioning of the `^SP` command requires a trial-and-error process, as it
depends primarily on print speed and label complexity.

The `^SP` command can be effectively used to determine the worst possible print quality. You can
determine whether using the `^SP` command is appropriate for the particular application by using this
procedure.

If you send the label format up to the first `^SP` command and then wait for printing to stop before sending
the next segment, the printed label is a sample of the worst possible print quality. It drops any field that is
out of order.


If the procedure above is used, the end of the label format must be:

```
^SP#^FS

```

**Format:** `^SPa`






|Parameters|Details|
|---|---|
|`a =` dot row to start<br>printing|**Values:** `0` to`32000`<br>**Default:** `0`|



**Example:** In this example, a label 800 dot rows in length uses `^SP500` . Segment 1 prints while commands
in Segment 2 are being received and formatted.


343


ZPL Commands


344