# ^SZ




ZPL Commands


The `^SZ` command is used to select the programming language used by the printer. This command gives
you the ability to print labels formatted in both ZPL and ZPL II.


**Set ZPL Mode**

This command remains active until another `^SZ` command is sent to the printer or the printer is turned off.

**Format:** `^SZa`

|Parameters|Details|
|---|---|
|`a =` ZPL version|**Values:**<br>`1 =` ZPL<br>`2 =`  ZPL II<br>**Default:** `2`|



**Comments:** If the parameter is missing or invalid, the command is ignored.


353