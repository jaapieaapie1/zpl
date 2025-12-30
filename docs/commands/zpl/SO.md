# ^SO




ZPL Commands


The `^SO` command is used to set the secondary and the tertiary offset from the primary Real-Time Clock.


**Set Offset (for Real-Time Clock)**


**NOTE:** For each label only one `^SO2` command can be used. If more than one offset is required,
`^SO3` must be used.

**Format:** `^SOa,b,c,d,e,f,g`

|Parameters|Details|
|---|---|
|`a =` clock set|**Values:**<br>`2 =` secondary<br>`3 =` third<br>**Default:** value must be specified|
|`b =` months offset|**Values:** `–32000` to`32000`<br>**Default:** `0`|
|`c =` days offset|**Values:** `–32000` to`32000`<br>**Default:** `0`|
|`d =` years offset|**Values:** `–32000` to`32000`<br>**Default:** `0`|
|`e =` hours offset|**Values:** `–32000` to`32000`<br>**Default:** `0`|
|`f =` minutes offset|**Values:** `–32000` to`32000`<br>**Default:** `0`|
|`g =` seconds offset|**Values:** `–32000` to`32000`<br>**Default:** `0`|



For more detail on set offset, see Real Time Clock on page 1595.


342