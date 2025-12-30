# ^FN




ZPL Commands


The `^FN` command numbers the data fields. This command is used in both `^DF` (Store Format) and `^XF`
(Recall Format) commands.


**Field Number**

In a stored format, use the `^FN` command where you would normally use the `^FD` (Field Data) command. In
recalling the stored format, use `^FN` in conjunction with the `^FD` command.

The optional `"a"` parameter can be used with the KDU Plus to cause prompts to be displayed on the KDU
unit. Also, when the Print on Label link is selected on the Directory page of ZebraLink enabled printers the
field prompt displays.


The number of fields and data that can be stored is dependent in the available printer memory.


**NOTE:** The maximum number of `^FN` commands that can be used depends on the amount of
data that is placed in the fields on the label. It is recommended to use 400 or fewer fields.

**Format:** `^FN#"a"`







|Parameters|Details|
|---|---|
|`# =` number to be<br>assigned to the field|**Values:** `0` to`9999`<br>**Default:** `0`|
|`"a" =` optional<br>parameter*|**Values:** 255 alphanumeric characters maximum (a-z,A-Z,1-9 and space)<br>**Default:** optional parameter|
|* This parameter is only available on printers with firmware V50.13.2, V53.15.5Z, V60.13.0.1, or later. For a<br>complete example of the`^DF` and`^XF` command, seeExercise 6: ^DF and ^XF - Download Format and<br>Recall Format.|* This parameter is only available on printers with firmware V50.13.2, V53.15.5Z, V60.13.0.1, or later. For a<br>complete example of the`^DF` and`^XF` command, seeExercise 6: ^DF and ^XF - Download Format and<br>Recall Format.|


**Comments:**

- The same `^FN` value can be stored with several different fields.

- If a label format contains a field with `^FN` and `^FD`, the data in that field prints for any other field
containing the same `^FN` value.

- For the `"a"` parameter to function as a prompt the characters used in the `"a"` parameter must be
surrounded by double quotes (see example).

The `^FN1"Name"` would result in `"Name"` being used as the prompt on the KDU unit.


200