# ^FT




ZPL Commands


The `^FT` command sets the field position, relative to the home position of the label designated by the `^LH`
command. The typesetting origin of the field is fixed with respect to the contents of the field and does not
change with rotation.


**Field Typeset**


**NOTE:** The `^FT` command is capable of the concatenation of fields.


**Format:** `^FTx,y,z`






|Parameters|Details|
|---|---|
|`x =` x-axis location (in<br>dots)|**Values:** `0` to`32000`<br>**Default:** position after the last formatted text field|
|`y =` y-axis location (in<br>dots)|**Values:** `0` to`32000`<br>**Default:** position after the last formatted text field|
|`z =` justification<br>The`z` parameter is<br>only supported in<br>firmware version V60.14.x,<br>V50.14.x, or later.|**Values:**<br>`0 =` left justification<br>`1 =` right justification<br>`2 =` auto justification (script dependent)<br>**Default:** last accepted`^FW` value or`^FW` default<br>The auto-justification option may cause unexpected results if variable fields<br>or bidirectional text are used with`^FT`. For best results with bidirectional<br>text and/or variable fields, use either the left or right justification options.|



















|Table 7 Typeset Ju|ustification|Col3|
|---|---|---|
|Left Justified|Text|For examples, seeField Interactions on page 1588.|
|Left Justified|Bar<br>Codes|Origin is the base of barcode, at the left edge|
|Left Justified|Graphic<br>Boxes|The origin is bottom-left corner of the box|
|Left Justified|Images|Origin is bottom-left corner of the image area|
|Right Justified|Text|For examples, seeField Interactions on page 1588.|
|Right Justified|Bar<br>Codes|Origin is the base of barcode, at the right edge|
|Right Justified|Graphic<br>Boxes|Origin is bottom-right corner of the box|
|Right Justified|Images|Origin is bottom-right corner of the image area|


**Example:** This is an example of the `^FT` command and concatenation:


205


ZPL Commands


When a coordinate is missing, the position following the last formatted field is assumed. This **remembering**
simplifies field positioning with respect to other fields. Once the first field is positioned, other fields follow
automatically.

There are several instances where using the `^FT` command without specifying `x` and `y` parameters is not
recommended:


- when positioning the first field in a label format

- at any time with the `^FN` (Field Number) command

- following an `^SN` (Serialization Data) command

- variable data


- bidirectional text


The right typeset justified is available only for printers with firmware version V60.14.x, V50.14.x, or later.

This command interacts with the field direction parameters of `^FP` and with the rotation parameter of `^A` .
For output and code examples, see Field Interactions on page 1588.


206