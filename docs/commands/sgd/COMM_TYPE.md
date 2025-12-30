# comm.type



This printer setting determines the behavior of the serial port interface. It selects one of three serial
communication states: DTE, DCE or Autodetect.


**Setvar**


To instruct the printer to change the serial port interface type:

```
       ! U1 setvar "comm.type" "value"

```

**Values**

              - `"auto" =` Autodetect

              - `"dte" =` Force DTE (Tx on pin 2)

              - `"dce" =` Force DCE (Rx on pin 2)

**Default**
```
          "auto"

```

**Getvar**


To instruct the printer to respond with the current serial port interface type:

```
       ! U1 getvar "comm.type"

```

**Examples**

In this example, the `getvar` retrieves the serial port communications state.

```
       ! U1 getvar "comm.type"

```

This `setvar` example sets the communications port state to auto-detect.

```
       ! U1 setvar "comm.type" "auto"

```

654


SGD Printer Commands