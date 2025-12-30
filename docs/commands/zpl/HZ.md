# ^HZ




ZPL Commands


The `^HZ` command is used for returning printer description information in XML format. The printer returns
information on format parameters, object directories, individual object data, and print status information.


**Display Description Information**

**Format:** `^HZb`



|Parameters|Details|
|---|---|
|`b =` display description to<br>return|**Values:**<br>`a =` display all information<br>`f =` display printer format setting information<br>`l =` display object directory listing information<br>`o =` display individual object data information<br>`r =` display printer status information<br>**Default:** if the value is missing or invalid, the command is ignored|


**Format:** `^HZO,d:o.x,l`
















|Parameters|Details|
|---|---|
|`d =` location of stored<br>object|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` object name|**Values:** `1` to`8`, or`1` to`16` alphanumeric characters based on parameter`l`.<br>**Default:** if a name is not specified,`UNKNOWN` is used.|
|`x =` extension|Supported extensions for objects (parameter`o`) include:<br>`.FNT` — font<br>`.GRF` — graphic<br>`.PNG` — compressed graphic<br>`.ZPL` — stored format<br>`.DAT` — encoding table<br>`.ZOB` — downloadable object<br>`.STO` — Alert data file|
|`l =` long filename<br>support|**Values:**<br>`Y =` Yes<br>If`Y`, the object data stores the filename as 16 characters. The data is only<br>compatible with firmware version V60.13.0.5, or later.<br>`N =` No<br>If`N`, the object data stores the filename as 8 characters. The data is forward<br>and backward compatible with all versions of firmware.<br>**Default:** `N`|



**Example:** This example shows the object data information for the object `SAMPLE.GRF` located on `R:` .

```
^XA

```

243


```
^HZO,R:SAMPLE.GRF
^XZ

```


ZPL Commands


244