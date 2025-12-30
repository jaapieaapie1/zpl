# ^GS




ZPL Commands


The `^GS` command enables you to generate the registered trademark, copyright symbol, and other
symbols.


**Graphic Symbol**

**Format:** `^GSo,h,w`






|Parameters|Details|
|---|---|
|`o =` field orientation|**Values:**<br>`N =` normal`R =` rotate 90 degrees clockwise`I =` inverted 180 degrees`B =`<br>bottom-up, 270 degrees**Default:** N or last`^FW` value|
|h `=` character height<br>proportional to width (in<br>dots)|**Values:** 0 to 32000<br>**Default:** last`^CF` value|
|w `=` character width<br>proportional to height (in<br>dots)|**Values:** 0 to 32000<br>**Default:** last`^CF` value|



Use the `^GS` command followed by `^FD` and the appropriate character (A through E) within the field data to
generate the desired character:


217