# rfid.reader_1.firmware_version



This command returns the RFID reader firmware version.


**Getvar**


To return the RFID reader firmware version:

```
       ! U1 getvar "rfid.reader_1.firmware_version"

```

**Example**


This example shows the response you receive when an RFID reader is attached:

```
       ! U1 getvar "rfid.reader_1.firmware_version"

```

If an RFID reader is present and connected, you get the firmware version in the following format:
```
       "xx.xx.xx.xx"
```

If there is no RFID reader or if the reader is not connected correctly, the response is blank.


1524


SGD RFID Commands