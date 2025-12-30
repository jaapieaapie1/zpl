# odometer.rfid.void_resettable



This command resets the RFID void label counter to zero.


**Setvar**


To set the RFID void counter to zero:

```
       ! U1 setvar "odometer.rfid.void_resettable" "value"

```

**Values**
```
          "0"

```

**Getvar**


To respond with the current RFID void counter value:

```
       ! U1 getvar "odometer.rfid.void_resettable"

```

**Example**

This `setvar` example shows how the counter portion of the printer configuration labels looks when the
RFID void counter is reset by sending:

```
       ! U1 setvar "odometer.rfid.valid_resettable" "0"

```

**Figure 20** Before


929


**Figure 21** After



SGD Printer Commands


930




SGD Printer Commands