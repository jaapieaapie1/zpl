# ^SE




ZPL Commands


The `^SE` command is used to select the desired ZPL or ZPL II encoding table.


**Select Encoding Table**
```
^SEd:o.x

```






|Parameters|Details|
|---|---|
|`d =` location of encoding<br>table|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` name of encoding<br>table|**Values:** `1` to`8` alphanumeric characters<br>**Default:** a value must be specified.|
|`x =` extension|**Fixed Value:** `.DAT`|


The encoding tables are provided with the font card or downloaded in flash with the font. The table
appears as `XXXXXXX.DAT` in a directory label printed by the ZPL commands.

The most active encoding table is indicated by the `*` on the directory label.

**Example:**

```
^XA^WD*:*.*^XZ

```

333