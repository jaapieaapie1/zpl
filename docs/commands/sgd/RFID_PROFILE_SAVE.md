# rfid.profile_save



This command is for users to create and save the RFID calibration profile onto the `E:` drive when needed.


**Setvar**


To set the IPP mode:

```
       ! U1 setvar "ip.ipp.mode" "value"

```

**Values**
Maximum of 8-byte filename length


**Example**

This `setvar` example shows the value set to `"RFIDCAL1.RPF"` .

```
       ! U1 setvar "rfid.profile_save" "RFIDCAL1.RPF"

```

1516


SGD RFID Commands