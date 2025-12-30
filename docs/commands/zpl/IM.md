# ^IM




ZPL Commands


The `^IM` command performs a direct move of an image from storage area into the bitmap. The command is
identical to the `^XG` command (Recall Graphic), except there are no sizing parameters.


**Image Move**

**Format:** `^IMd:o.x`







|Parameters|Details|
|---|---|
|`d =` location of stored<br>object|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** search priority|
|`o =` object name|**Values:** `1` to`8` alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension|**Fixed Value:** `.GRF`, `.PNG`|


**Example:** This example moves the image `SAMPLE.GRF` from DRAM and prints it in several locations in its
original size.

```
^XA

^FO100,100^IMR:SAMPLE.GRF^FS
^FO100,200^IMR:SAMPLE.GRF^FS
^FO100,300^IMR:SAMPLE.GRF^FS
^FO100,400^IMR:SAMPLE.GRF^FS
^FO100,500^IMR:SAMPLE.GRF^FS
^XZ

```

**Comments:** By using the `^FO` command, the graphic image can be positioned anywhere on the label.

The difference between `^IM` and `^XG` : `^IM` does not have magnification, and therefore might require less
formatting time. However, to take advantage of this, the image must be at a 8-, or 32-bit boundary.
16-,


248