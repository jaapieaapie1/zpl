# zpl.caret




SGD Printer Commands


This command is used to change the format command prefix for ZPL commands. The default prefix is the
caret ( `^` )

This command is equivalent to the `~CC` and `^CC` ZPL commands.


**Setvar**


To set the command prefix:

```
! U1 setvar "zpl.caret" "value"

```

**Values**
Any ASCII character

**Default**
```
   "^"

```

**Getvar**


To retrieve the current format command prefix:

```
! U1 getvar "zpl.caret"

```

**Example**

This `setvar` example changes the format prefix to a forward slash `"/"`

```
! U1 setvar "zpl.caret" "/"

```

1071


SGD Printer Commands