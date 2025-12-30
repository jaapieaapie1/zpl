# ^HW




ZPL Commands


`^HW` is used to transmit a directory listing of objects in a specific memory area (storage device) back to the
host device. This command returns a formatted ASCII string of object names to the host.


**Host Directory List**


Each object is listed on a line and has a fixed length. The total length of a line is also fixed. Each line listing
an object begins with the asterisk (*) followed by a blank space. There are eight spaces for the object
name, followed by a period and three spaces for the extension. The extension is followed by two blank
spaces, six spaces for the object size, two blank spaces, and three spaces for option flags (reserved for
future use). The format looks like this:

```
<STX><CR><LF>
DIR R: <CR><LF>
*Name.ext(2sp.)(6 obj. sz.)(2sp.)(3 option flags)
*Name.ext(2sp.)(6 obj. sz.)(2sp.)(3 option flags)
<CR><LF>
-xxxxxxx bytes free
<CR><LF>
<ETX>
<STX> = start of text
<CR><LR> = carriage return/line feed
<ETX> = end on text

```

The command might be used in a stand-alone file to be issued to the printer at any time. The printer
returns the directory listing as soon as possible, based on other tasks it might be performing when the
command is received.


This command, like all ^ (caret) commands, is processed in the order that it is received by the printer.

**Format:** `^HWd:o.x`












|Parameters|Details|
|---|---|
|`d =` location to retrieve<br>object listing|**Values:** `R:`, `E:`, `B:`, `A:`and`Z:`<br>**Default:** `R:`|
|`o =` object name|**Values:** `1` to`8` alphanumeric characters<br>**Default:** asterisk (`*`). A question mark (`?`) can also be used.|
|`x =` extension|**Values:** any extension conforming to Zebra conventions<br>**Default:** asterisk (`*`). A question mark (`?`) can also be used.|
|`f =` format<br>The`f` parameter is only<br>supported in firmware<br>version V60.16.0Z and<br>V53.16.0Z or later.|**Values:**<br>`c =` column format<br>`d =` default format<br>**Default:** `d`|



**Example:** Listed is an example of the `^HW` command to retrieve from information `R:`

```
^XA

```

240




ZPL Commands

```
^HWR:*.*
^XZ

```

The printer returned this information as the Host Directory Listing: `-DIR R:*.*`

```
*R:ARIALN1.FNT 49140
*R:ARIALN2.FNT 49140
*R:ARIALN3.FNT 49140
*R:ARIALN4.FNT 49140
*R:ARIALN.FNT 49140
*R:ZEBRA.GRF 8420
-794292 bytes free R:RAM

```

241