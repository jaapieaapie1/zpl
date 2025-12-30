# file.run




SGD Printer Commands


This command instructs the printer to send a specified file to the parser.


**Setvar**


To instruct the printer to send a specified file to the parser:

```
! U1 setvar "file.run" "values"

```

**Values**


drive:filename.extension


**NOTE:** Be sure to always specify the memory location.


**Do**


To instruct the printer to send a specified file to the parser:

```
! U1 do "file.run" "value"

```

**Values**


drive:filename.extension


**NOTE:** Be sure to always specify the memory location.


**Example**

This `setvar` example will send the file "text.zpl" stored in RAM to the parser.

```
! U1 setvar "file.run" "R:text.zpl"

```

827


SGD Printer Commands