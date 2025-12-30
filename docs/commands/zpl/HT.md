# ^HT




ZPL Commands


The `^HT` command receives the complete list of font links over a communication port.


**Host Linked Fonts List**


This command is available only for printers with firmware version V60.14.x, V50.14.x, or later.

The `SWISS.721.TTF` is the base font, `ANMDJ.TTF` is the first linked font, and `MSGOTHIC.TTF` is the
second linked font:


This is the code that was used to establish the font links:

```
^XA
^FLE:ANMDJ.TTF,E:SWISS721.TTF,1^FS
^FLE:MSGOTHIC.TTF,E:SWISS721.TTF,1^FS
^XZ

```

237