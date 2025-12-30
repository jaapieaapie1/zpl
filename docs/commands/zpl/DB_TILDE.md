# ~DB




ZPL Commands


The `~DB` command sets the printer to receive a downloaded bitmap font and defines native cell size,
baseline, space size, and copyright.


**Download Bitmap Font**


This command consists of two portions, a ZPL II command defining the font and a structured data segment
that defines each character of the font.

**Format:** `~DBd:o.x,a,h,w,base,space,#char,©,data`







|Parameters|Details|
|---|---|
|`d =` drive to store font|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` name of font|**Values:** 1 to 8 alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension|**Format:** `.FNT`|
|`a =` orientation of native<br>font|**Fixed Value:** normal|
|`h =` maximum height of<br>cell (in dots)|**Values:** `1` to`32000`<br>**Default:** a value must be specified|
|`w =` maximum width of<br>cell (in dots)|**Values:** `1` to`32000`<br>**Default:** a value must be specified|
|`base =` dots from top of<br>cell to character baseline|**Values:** `1` to`32000`<br>**Default:** a value must be specified|
|`space =` width of space<br>or non-existent characters|**Values:** `1` to`32000`<br>**Default:** a value must be specified|
|`#char =` number of<br>characters in font|**Values:** `1` to`256` (must match the characters being downloaded)<br>**Default:** a value must be specified|
|`© =` copyright holder|**Values:** `1` to`63` alphanumeric characters<br>**Default:** a value must be specified|


170




ZPL Commands






|Parameters|Details|
|---|---|
|`data =` structured ASCII<br>data that defines each<br>character in the font|The # symbol signifies character code parameters, which are separated<br>with periods. The character code is from 1 to 4 characters to allow for large<br>international character sets to be downloaded to the printer.<br>The data structure is:<br>`#xxxx.h.w.x.y.i.data`<br>`#xxxx =` character code<br>`h =` bitmap height (in dot rows)<br>`w =` bitmap width (in dot rows)<br>`x =` x-offset (in dots)<br>`y =` y-offset (in dots)<br>`i =` typesetting motion displacement (width, including inter character gap<br>of a particular character in the font)<br>`data =` hexadecimal bitmap description|



**Example:** This is an example of how to use the `~DB` command. It shows the first two characters of a font
being downloaded to DRAM.

```
~DBR:TIMES.FNT,N,5,24,3,10,2,ZEBRA 1992,
#0025.5.16.2.5.18.
OOFF
OOFF
FFOO
FFOO
FFFF
#0037.4.24.3.6.26.
OOFFOO
OFOOFO
OFOOFO
OOFFOO

```

171