# ~DG




ZPL Commands


The `~DG` command downloads an ASCII Hex representation of a graphic image. If `.GRF` is not the specified
file extension, `.GRF` is automatically appended.


**Download Graphics**


For more saving and loading options when downloading files, see ~DY on page 181.

**Format:** `~DGd:o.x,t,w,data`












|Parameters|Details|
|---|---|
|`d =` device to store the<br>image|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` image name|**Values:**<br>`1` to`8` alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension|**Format:** `.GRF`|
|`t =` total number of bytes<br>in the graphic|See the formula in the examples below.|
|`w =` number of bytes per<br>row|See the formula in the examples below.|
|`data =` ASCII<br>hexadecimal string<br>defining image|The data string defines the image and is an ASCII hexadecimal<br>representation of the image. Each character represents a horizontal nibble<br>of four dots.|



This is the key for the examples that follow:

x `=` width of the graphic in millimeters

y `=` height of the graphic in millimeters

z `=` dots/mm `=` print density of the printer being programmed

8 `=` bits/byte

These are some examples related to the `~DG` command:

**Example:** To determine the `t` parameter use this formula:

(xz/8) x yz = totalbytes


**Example:** To determine the correct t parameter for a graphic 8 mm wide, 16 mm high, and a print density of
8 dots/mm, use this formula:


8 x 128 = 1024


t = 1024


**Raise any portion of a byte to the next whole byte.**

**Example:** To determine the `w` parameter (the width in terms of bytes per row), use this formula:

xz/8 = (totalbytes)/(row)


w = 8

**Example:** To determine the correct `w` parameter for a graphic 8 mm wide and a print density of 8 dots/mm,
use this formula:


175


ZPL Commands


(8 x 8)/8 = 8 bytes


w = 8


**Raise any portion of a byte to the next whole byte.**

Parameter `w` is the first value in the `t` calculation.

The data parameter is a string of hexadecimal numbers sent as a representation of the graphic image.
Each hexadecimal character represents a horizontal nibble of four dots. For example, if the first four
dots of the graphic image are white and the next four black, the dot-by-dot binary code is 00001111. The
hexadecimal representation of this binary value is 0F. The entire graphic image is coded in this way, and
the complete graphic image is sent as one continuous string of hexadecimal values.

**Example:** This is an example of using the `~DG` command to load a checkerboard pattern into DRAM. The
name used to store the graphic is `SAMPLE.GRF` :


**Comments:** Do not use spaces or periods when naming your graphics. Always use different names for
different graphics.


If two graphics with the same name are sent to the printer, the first graphic is erased and replaced by the
second graphic.


176