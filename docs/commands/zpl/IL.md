# ^IL




ZPL Commands


The `^IL` command is used at the beginning of a label format to load a stored image of a format and merge
it with additional data. The image is always positioned at `^FO0,0` .


**Image Load**


**IMPORTANT:** See ^IS.


Using this technique to overlay the image of constant information with variable data greatly increases the
throughput of the label format.

**Format:** `^ILd:o.x`







|Parameters|Details|
|---|---|
|`d =` location of the stored<br>object|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` object name|**Values:** `1` to`8` alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension|**Fixed Value:** `.GRF`, `.PNG`|


**Example**

This example recalls the stored image `SAMPLE2.GRF` from DRAM and overlays it with the additional data.
The graphic was stored using the `^IS` command. For the stored label format, see the ^IS command.


247