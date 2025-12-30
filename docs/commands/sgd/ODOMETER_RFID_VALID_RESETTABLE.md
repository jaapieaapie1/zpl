# odometer.rfid.valid_resettable



This command resets the RFID valid label counter to zero.


**Setvar**


To set the RFID valid counter to zero:

```
       ! U1 setvar "odometer.rfid.valid_resettable" "value"

```

**Values**
```
          "0"

```

**Getvar**


To respond with the current RFID valid counter value:

```
       ! U1 getvar "odometer.rfid.valid_resettable"

```

**Example**

This `setvar` example shows how the counter portion of the printer configuration labels looks when the
RFID valid counter is reset by sending:

```
       ! U1 setvar "odometer.rfid.valid_resettable" "0"

```

**Figure 18** Before


927


**Figure 19** After



SGD Printer Commands


928


SGD Printer Commands