# ^LL




ZPL Commands


The `^LL` command defines the length of the label. This command is necessary when using continuous
media (media not divided into separate labels by gaps, spaces, notches, slots, or holes). This command is
not persistent across a power cycle unless a `^JUS` is received.

See also zpl.label_length_always on page 1075.


**Label Length**

To affect the current label and be compatible with existing printers, `^LL` must come before the first `^FS`
(Field Separator) command. Once you have issued `^LL`, the setting is retained until you turn off the printer
or send a new `^LL` command.

**Format:** `^LLy.x`

|Parameters|Details|
|---|---|
|`y =`|Defines the label length.<br>**Values:** `1` to`32000`, not to exceed the maximum label size.<br>While the printer accepts any value for this parameter, the amount of<br>memory installed determines the maximum length of the label.<br>**Default:** typically set through the LCD (if applicable), or to the maximum<br>label length capability of the printer.|
|`x =`|Specifies whether the label length applies to all media, including Gap and<br>Mark.<br>**Values:** `N` or`Y`, `n` or`y` is also accepted.<br>•<br>`N` means that the`^LL` length applies only to continuous media.<br>•<br>`Y` means that the`^LL` length applies to all media, including Gap and<br>Mark.<br>**Default:** `N`. If no value is present, the current setting is left unchanged.|



**Comments:** These formulas can be used to determine the value of y:


**For 6 dot/mm printheads...** Label length in inches x 152.4 (dots/inch) = y

**For 8 dot/mm printheads...** Label length in inches x 203.2 (dots/inch) = y

**For 12 dot/mm printheads...** Label length in inches x 304.8 (dots/inch) = y



**For 24 dot/mm**
**printheads...**



Label length in inches x 609.6 (dots/inch) = y



Values for `y` depend on the memory size. If the entered value for `y` exceeds the acceptable limits, the
bottom of the label is cut off. The label also shifts down from top to bottom.

If multiple `^LL` commands are issued in the same label format, the last `^LL` command affects the next label
unless it is prior to the first `^FS` .

This command is ignored on the HC100™ printer.


294