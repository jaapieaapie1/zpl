# ^IS




ZPL Commands


The `^IS` command is used within a label format to save that format as a graphic image rather than as a ZPL
II script. It is typically used toward the end of a script. The saved image can later be recalled with virtually
no formatting time and overlaid with variable data to form a complete label.


**Image Save**


Using this technique to overlay the image of constant information with the variable data greatly increases
the throughput of the label format.


**IMPORTANT:** See ^IL.


**Format:** `^ISd:o.x,p`









|Parameters|Details|
|---|---|
|`d =` location of the stored<br>object|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` object name|**Values:** `1` to`8` alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension|**Values:** `.GRF` or`.PNG`<br>**Default:** `.GRF`|
|`p =` print image after<br>storing|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|


**Example**





This is an example of using the `^IS` command to save a label format to DRAM. The name used to store the
graphic is `SAMPLE2.GRF` .


249


ZPL Commands


250