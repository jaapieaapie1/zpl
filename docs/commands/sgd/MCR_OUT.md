# mcr.out




SGD Printer Commands


Specifies the communication port which MCR (Mag Card Reader) output is sent to.


**Setvar**


To specify the communication port which MCR (Mag Card Reader) output is sent to:

```
! U1 setvar "mcr.out" "value"

```

**Values**

`"active"` means the data is sent out over the same port that the command was received on.

If “multiple” is specified in the option string of the `mcr.enable` command, data will continue to be
be sent to the port defined by this command.

`"alert"` means the data will be forwarded as a weblink alert.

**Default**
```
   "active"

```

845


SGD Printer Commands