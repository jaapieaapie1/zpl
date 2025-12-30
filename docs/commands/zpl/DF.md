# ^DF




ZPL Commands


The `^DF` command saves ZPL II format commands as text strings to be later merged using `^XF` with
variable data. The format to be stored might contain field number ( `^FN` ) commands to be referenced when
recalled.


**Download Format**


While the use of stored formats reduces transmission time, no formatting time is saved—this command
saves ZPL II as text strings formatted at print time.

Enter the `^DF` stored format command immediately after the `^XA` command, then enter the format
commands to be saved.

**Format:** `^DFd:o.x`







|Parameters|Details|
|---|---|
|`d =` device to store the<br>image|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` image name|**Values:** 1 to 16 alphanumeric characters with a file type of 1 to 3<br>alphanumeric characters separated by a "."<br>**Default:** if a name is not specified, UNKNOWN is used.|
|`x =` extension|**Format:** `.ZPL`|


For a complete example of the `^DF` and `^XF` command, see Exercise 6: ^DF and ^XF — Download Format
and Recall Format on page 57.


**Example**

This example is generated using the `^XF` command to recall this format:


174