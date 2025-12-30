# rfid.reader_1.hardware_version



This command returns the RFID reader hardware version.


**Getvar**


To return the RFID reader hardware version:

```
       ! U1 getvar "rfid.reader_1.hardware_version"

```

**Example**


This example shows the response you receive when an RFID reader is attached:

```
       ! U1 getvar "rfid.reader_1.hardware_version"

```

If an RFID reader is present and connected, you get the hardware version in the following format:
```
       "xx.xx.xx.xx"
```

If there is no RFID reader or if the reader is not connected correctly, the response is blank.


1525


SGD RFID Commands