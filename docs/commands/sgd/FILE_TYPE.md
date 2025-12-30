# file.type




SGD Printer Commands


This command displays the contents of the specified file.


**Setvar**


To instruct the printer to display the content of a file:

```
! U1 setvar "file.type" "value"

```

The contents are displayed on the same port as the command was received.


**Values**

the drive letter, file name, file extension, such as `R:TEST.ZPL`


**NOTE:** Be sure to always specify the memory location.


**Do**


To display the content of a specified file:
```
! U1 do "file.type" "value"
```

**Values**

The drive letter, file name, file extension, such as `R:TEST.ZPL`


**NOTE:** Be sure to always specify the memory location.


**Example**

This `setvar` example shows the value set to `"R:TEST.ZPL"` .

```
! U1 setvar "file.type" "R:TEST.ZPL"

```

When the `setvar` value is set to `"R:TEST.ZPL"`, the contents of the file `TEST.ZPL` located on the `R:`
drive will be displayed.


826