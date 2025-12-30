# comm.parity



This command sets the printer’s communication parity.


Once the printer’s communication port parameters have been changed, the host terminal must also be
configured to match the new printer settings before the host can communicate again.


**Setvar**


To instruct the printer to set the communication port parity:

```
       ! U1 setvar "comm.parity" "value"

```

**Values**

              - `"N"`               - None

              - `"E"`               - Even

              - `"O"`               - Odd


**Getvar**


To instruct the printer to respond with the currently set printer parity:

```
       ! U1 getvar "comm.parity"

```

**Examples**

In this example, the `getvar` retrieves the currently set printer parity.

```
       ! U1 getvar "comm.parity"

```

This `setvar` example sets the parity to None.

```
       ! U1 setvar "comm.parity" "N"

```

655


SGD Printer Commands