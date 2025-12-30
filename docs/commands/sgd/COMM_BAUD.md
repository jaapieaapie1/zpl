# comm.baud



This command refers to the printer’s comm (cable) baud rate.


**NOTE:** Once the printer’s communication port parameters have been changed, the host terminal
must also be configured to match the new printer settings before the host can communicate
again.


**Setvar**


To instruct the printer to change the baud rate:

```
       ! U1 setvar "comm.baud" "value"

```

**Values**

              - `"9600"`

              - `"19200"`

              - `"38400"`

              - `"57600"`

              - `"115200"`

**Default**
```
          "19200"

```

**Getvar**


To instruct the printer to respond with the currently set printer baud rate:

```
       ! U1 getvar "comm.baud"

```

**Examples**

In this example, the `getvar` retrieves the current baud rate.

```
       ! U1 getvar "comm.baud"

```

This `setvar` example sets the communications baud rate to 19200 BPS.

```
       ! U1 setvar "comm.baud" "19200"

```

650




SGD Printer Commands