# ~DU




ZPL Commands


Some international fonts, such as Asian fonts, have more than 256 printable characters. These fonts are
supported as **large TrueType fonts** and are downloaded to the printer with the `~DU` command.


**Download Unbounded TrueType Font**


Use ZTools to convert the large TrueType fonts to a Zebra-downloadable format.

The Field Block ( `^FB` ) command cannot support the large TrueType fonts.

**Format:** `~DUd:o.x,s,data`

|Parameters|Details|
|---|---|
|`d =` font location|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` font name|**Values:** 1 to 8 alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension|**Format:** `.FNT`|
|s `=` font size|**Values:** the number of memory bytes required to hold the Zebra-<br>downloadable format of the font<br>**Default:** if no data is entered, the command is ignored|
|`data =` data string|**Values:** a string of ASCII hexadecimal values (two hexadecimal digits/byte).<br>The total number of two-digit values must match parameter`s`.<br>**Default:** if no data is entered, the command is ignored|



**Example:** This is an example of how to download an unbounded true type font:

```
~DUR:KANJI,86753,60CA017B0CE7...

```

(86753 two-digit hexadecimal values)


For similar commands, see ~DS on page 178, ~DT on page 179, and ~DY on page 181.


180