# ~DY




ZPL Commands


The `~DY` command downloads to the printer graphic objects or fonts in any supported format. This
command can be used in place of `~DG` for more saving and loading options. `~DY` is the preferred command
to download TrueType fonts on printers with firmware later than X.13. It is faster than `~DU` . The `~DY`
command also supports downloading wireless certificate files.


**Download Objects**


**NOTE:** Note: When using certificate files, your printer supports:


- Using Privacy Enhanced Mail (PEM) formatted certificate files.


- Using the client certificate and private key as two files, each downloaded separately.


- Using exportable PAC files for EAP-FAST.


- Zebra recommends using Linear style memory devices for storing larger objects.


**Format:** ~DVd:f,b,x,t,w,data











|Parameters|Details|
|---|---|
|`d =` file location<br>`.NRD` and`.PAC` files<br>reside on E: in firmware<br>versions V60.15.x,<br>V50.15.x, or later.|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`f =` file name|**Values:** 1 to 8 alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`b =` format downloaded<br>in data field<br>`.TTE` and`.TTF` are only<br>supported in firmware<br>versions V60.14.x,<br>V50.14.x, or later.|**Values:**<br>`A =` uncompressed (ZB64, ASCII)<br>`B =` uncompressed (`.TTE`, `.TTF`, binary)<br>`C =` AR-compressed (used only by Zebra’s BAR-ONE® v5)<br>`P =` portable network graphic (`.PNG`) - ZB64 encoded<br>**Default:** a value must be specified|


181


ZPL Commands





|Parameters|Details|
|---|---|
|`x =` extension of stored<br>file<br>`.TTE` and`.OTF` are only<br>supported in firmware<br>versions V60.14.x,<br>V50.14.x, or later.<br>`.NRD` and`.PAC` are only<br>supported in firmware<br>versions V60.15.x,<br>V50.15.x, or later.|**Values:**<br>`B =` bitmap<br>`E =` TrueType Extension (`.TTE`)<br>`G =` raw bitmap (`.GRF`)<br>`P =` store as compressed (`.PNG`)<br>`T =` TrueType (`.TTF`) or OpenType (`.OTF`)<br>`X =` Paintbrush (`.PCX`)<br>`NRD =` Non Readable File (`.NRD`)<br>`PAC =` Protected Access Credential (`.PAC`)<br>`C =` User defined menu file (WML)<br>`F =` User defined webpage file (HTM)<br>`H =` Printer feedback file (GET)<br>**Default:** a value other than the accepted values defaults to`.GRF`|
|t `=` total number of bytes<br>in file<br>**Figure 16**<br>`.TTE` is only supported<br>in firmware versions<br>V60.14.x, V50.14.x, or<br>later.|**Values:**<br>`.BMP`<br>This parameter refers to the actual size of the file, not the amount of disk<br>space.<br>`.GRF` images: the size after decompression into memory<br>This parameter refers to the actual size of the file, not the amount of disk<br>space.<br>`.PCX`<br>This parameter refers to the actual size of the file, not the amount of disk<br>space.<br>`.PNG` images:<br>This parameter refers to the actual size of the file, not the amount of disk<br>space.<br>`.TTF`<br>This parameter refers to the actual size of the file, not the amount of disk<br>space.<br>`.TTE`<br>This parameter refers to the actual size of the file, not the amount of disk<br>space.|


182


ZPL Commands







|Parameters|Details|
|---|---|
|`w =` total number of bytes<br>per row<br>`.TTE` is only supported in<br>firmware version V60.14.x,<br>V50.14.x, or later.<br>`.NRD` and`.PAC` files are<br>supported in firmware<br>version V60.15.x,<br>V50.15.x, or later.|**Values:**<br>`.GRF` images: number of bytes per row<br>`.PNG` images: value ignored<br>`.TTF` images: value ignored<br>`.TTE` images: value ignored<br>`.NRD` images: value ignored<br>`.PAC` images: value ignored|
|`data =` data|ASCII hexadecimal encoding, ZB64, or binary data, depending on`b`.<br>`A, P =` ASCII hexadecimal or ZB64<br>`B, C =` binary<br>When binary data is sent, all control prefixes and flow control characters<br>are ignored until the total number of bytes needed for the graphic format is<br>received.|


**NOTE:** When transmitting fonts or graphics, the `~DY` command and the binary content can be
sent as two separate data streams. In cases where the `~DY` command and data content are
sent separately, the connection to the printer must be maintained until both the command and
data content have been sent. If the command and data content are sent separately, the data
light on the printer will remain lit until it receives all the data called for in the `~DY` command.
The download will be considered complete when the number of bytes called out in the `~DY`
command have been received.


For best results, graphic files must be monochrome (black and white) or dithered.


**Example:** This is an example of how to download a binary TrueType Font file of Size bytes using the name
fontfile.ttf and storing it to permanent flash memory on the printer:

```
~DYE:FONTFILE.TTF,B,T,SIZE,,

```

These examples show:

- that when the `^IM` command is used with the `^FO` command, the `^IM` command (see ^IM on page
248) moves the `logo.png` file from a storage area to the 0,0 position on the label. This is the ZPL
code:

```
^XA
^FO0,0^IMR:LOGO.PNG^FS
^XZ

```

- that when the `^IL` command (see ^IL on page 247) is used at the beginning of a label format, it loads
a stored image ( `logo.png` ) of a format and merges it with additional data. It is automatically positioned
at the 0,0 position of the label and does not require the `^FO` command. This is the ZPL code:

```
^XA
^ILR:LOGO.PNG

```

183


ZPL Commands

```
^X

```

**Comments:** For more information on ZB64 encoding and compression, see ZB64 Encoding and
Compression on page 1584.


These are some important things to know about this command in firmware version V60.14.x, V50.14.x, or
later:


- ZebraNet Bridge can be used to download fonts and graphics with this command.


- OpenType tables are only supported when downloading the font with this command

- OpenType fonts ( `.OTF` ) are supported if they are downloaded as a TrueType font. In the printer `.OTF`
fonts have the `.TTF` extension.


184