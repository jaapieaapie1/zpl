# ^WD




ZPL Commands


The `^WD` command is used to print a label listing bar codes, objects stored in DRAM, or fonts.


**Print Directory Label**


For bar codes, the list shows the name of the bar code. For fonts, the list shows the name of the font, the
number to use with `^A` command, and size. For objects stored in DRAM, the list shows the name of the
object, extension, size, and option flags. All lists are enclosed in a double-line box.

**Format:** `^WDd:o.x`






|Parameters|Details|
|---|---|
|`d =` source device —<br>optional|**Values:** `R:`, `E:`, `B:`, `A:` and`Z:`<br>**Default:** `R:`|
|`o =` object name —<br>optional|**Values:** `1` to`8` alphanumeric characters<br>**Default:** *  The use of a`?` (question mark) is also allowed.|
|`x =` extension — optional<br>`.TTF` and`.TTE` are<br>only supported in<br>firmware version V60.14.x,<br>V50.14.x, or later.|**Values:** any extension conforming to Zebra conventions<br>`.FNT =`font<br>`.BAR =` bar code<br>`.ZPL =` stored ZPL format<br>`.GRF =` GRF graphic<br>`.CO =` memory cache<br>`.DAT =` font encoding<br>`.BAS =` ZBI encrypted program<br>`.BAE =` ZBI encrypted program<br>`.STO =` data storage<br>`.PNG =` PNG graphic<br>` * =` all objects.<br>`TTF =` TrueType Font<br>`.TTE =` True Type Extension<br>**Default:** `*`  The use of a`?` (question mark) is also allowed.|



**Example:** To print a label listing all objects in DRAM, enter:

```
^XA
^WDR:*.*
^XZ

```

**Example:** To print a label listing all resident bar codes, enter:

```
^XA
^WDZ:*.BAR
^XZ

```

**Example:** To print a label listing all resident fonts, enter:


360




```
^XA
^WDZ:*.FNT
^XZ

```


ZPL Commands


361