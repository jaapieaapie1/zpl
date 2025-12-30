# ^HY




ZPL Commands


The `^HY` command is an extension of the `^HG` command. `^HY` is used to upload graphic objects from the
printer in any supported format.


**Upload Graphics**

**Format:** `^HYd:o.x`

|Parameters|Details|
|---|---|
|`d =` location of object|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** search priority|
|`o =` object name|**Values:** `1` to`8` alphanumeric characters<br>**Default:** an object name must be specified|
|`x =` extension|**Values:**<br>`G = .GRF` (raw bitmap format)`P = .PNG` (compressed bitmap format)<br>**Default:** format of stored storage|



**Comments:** The image is uploaded in the form of a `~DY` command. The data field of the returned `~DY`
command is always encoded in the ZB64 format.


242