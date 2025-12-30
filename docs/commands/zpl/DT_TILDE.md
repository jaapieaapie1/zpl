# ~DT




ZPL Commands


The `~DT` command in ZPL (Zebra Programming Language) is used to download a TrueType font to a
printer. This allows the printer to use a standard scalable font for printing instead of a built-in bitmap font.


**Download Bounded TrueType Font**


Use ZTools to convert a TrueType font to a Zebra-downloadable format. that has less than 256 characters
in it. To convert a font that has more than 256 characters, see ~DU on page 180. ZTools creates a
downloadable file that includes a `~DT` command. For information on converting and downloading Intellifont
information, see ~DS on page 178.

**Format:** `~DTd:o.x,s,data`

|Parameters|Details|
|---|---|
|`d =` font location|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` font name|**Values:** any valid TrueType name, up to 8 characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension|**Fixed Value:** `.DAT`|
|`s =` font size|**Values:** the number of memory bytes required to hold the Zebra-<br>downloadable format of the font<br>**Default:** if an incorrect value or no value is entered, the command is<br>ignored|
|`data =` data string|**Values:** a string of ASCII hexadecimal values (two hexadecimal digits/byte).<br>The total number of two-digit values must match parameter`s`.<br>**Default:** if no data is entered, the command is ignored|



**Example:** This is an example of how to download a true type font:

```
~DTR:FONT,52010,00AF01B0C65E...

```

(52010 two-digit hexadecimal values)


179