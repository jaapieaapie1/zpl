# ^MU




ZPL Commands


The `^MU` command sets the units of measurement the printer uses. `^MU` works on a field-by-field basis.
Once the mode of units is set, it carries over from field to field until a new mode of units is entered.


**Set Units of Measurement**

`^MU` also allows for printing at lower resolutions — 600 dpi printers are capable of printing at 300, 200, and
150 dpi; 300 dpi printers are capable of printing at 150 dpi.

**Format:** `^MUa,b,c`






|Parameters|Details|
|---|---|
|`a =` units|**Values:**<br>`D =` dots<br>`I =` inches<br>`M =` millimeters<br>**Default:** `D`|
|`b =` format base in dots<br>per inch|**Values:** `150`, `200`, `300`<br>**Default:** a value must be entered or the command is ignored|
|`c =` desired dots-per-inch<br>conversion|**Values:** `300`, `600`<br>**Default:** a value must be entered or the command is ignored|



**Example:** This is an example of Setting Units:


Assume 8 dot/millimeter (203 dot/inch) printer.


Field based on dots:

```
^MUd^FO100,100^GB1024,128,128^FS

```

Field based on millimeters:

```
^MUm^FO12.5,12.5^GB128,16,16^FS

```

Field based on inches:
```
^MUi^FO.493,.493^GB5.044,.631,.631^FS
```

**Example:** This is an example of Converting dpi Values:


`^MUd,150,300` Convert a 150 dpi format to a 300 dpi format with a base in dots.

`^MUd,150,600` Convert a 150 dpi format to a 600 dpi format with a base in dots.

`^MUd,200,600` Convert a 200 dpi format to a 600 dpi format with a base in dots:


To reset the conversion factor to the original format, enter matching values for parameters b and c:

```
^MUd,150,150
^MUd,200,200
^MUd,300,300

```

311


ZPL Commands

```
^MUd,600,600

```

**Comments:** This command should appear at the beginning of the label format to be in proper ZPL II format.
To turn the conversion off, enter matching values for parameter `b` and `c` .


312