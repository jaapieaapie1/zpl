# ^LF




ZPL Commands


The `^LF` command prints out a list of the linked fonts.


**List Font Links**


This command is available only for printers with firmware version V60.14.x, V50.14.x, or later.


**Example**

This example shows that `SWISS721.TTF` is the based font. `ANMDJ.TTF` is the first linked font, and
`MSGOTHIC.TTF` is the second linked extension:


This is the code that established the font links:

```
^XA
^FLE:ANMDJ.TTF,E:SWISS721.TTF,1^FS
^FLE:MSGOTHIC.TTF,E:SWISS721.TTF,1^FS
^XZ

```

292