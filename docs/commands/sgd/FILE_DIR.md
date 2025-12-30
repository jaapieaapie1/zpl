# file.dir




SGD Printer Commands


This command displays a directory listing on the same port the command was received.


**Setvar**


To set the directory name from which to retrieve files:

```
! U1 setvar "file.dir" "value"

```

**Values**


directory letter


**NOTE:** Be sure to always specify the memory location.


**Getvar**


To retrieve a directory listing of the specified directory:

```
! U1 getvar "file.dir"

```

**NOTE:** Be sure to always specify the memory location.


**Do**


To set the directory name from which to retrieve files:

```
! U1 do "file.dir" "value"

```

**Values**


directory letter


**NOTE:** Be sure to always specify the memory location.


**Example**

This `do` example shows the directory listing of the specified directory.

```
! U1 do "file.dir" "R:"
- DIR R:*.*
- 11172192 bytes free R: RAM

```

824


SGD Printer Commands