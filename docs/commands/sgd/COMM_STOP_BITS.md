# comm.stop_bits



This command refers to the communication port stop bits of the printer.


**NOTE:** Once the printer’s communication port parameters have been changed, the host terminal
must also be configured to match the new printer settings before the host can communicate
again.


**Setvar**

To instruct the printer to configure the `comm.port` stop bit value:

```
       ! U1 setvar "comm.stop_bits" "value"

```

**Values**

              - `"1"`

              - `"2"`

**Default**
```
          "1"

```

**Getvar**


To instruct the printer to respond with the currently set stop bit value:

```
       ! U1 getvar "comm.stop_bits"

```

**Example**

In this example, the `getvar` retrieves the currently set stop bit value.

```
       ! U1 getvar "comm.stop_bits"

```

This `setvar` example configures the `comm.port` for 1 stop bit.

```
       ! U1 setvar "comm.stop_bits" "1"

```

656


SGD Printer Commands