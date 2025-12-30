# rfid.tag.test



This command performs an RFID test. In the RFID test, the printer attempts to read and write to a
transponder that you place over the RFID antenna. Results are displayed on the printer’s control panel
display.


For more information about the RFID antenna location, refer to the RFID Programming Guide for your
[printer. A copy is available online at www.zebra.com/manuals.](http://www.zebra.com/manuals)


In the slow version of the RFID test, the printer first displays the hardware version, the reader firmware
version, and the program position.


**NOTE:** This command is valid only on RP4T printers.


**Setvar**


To set the programming position:

```
       ! U1 setvar "rfid.tag.test" "value"

```

**Values**
```
          "quick"
          "slow"

```

**Example**

This `setvar` example performs a quick RFID test, which shows a pass or fail message.

```
       ! U1 setvar "rfid.tag.test" "quick"

```

This `setvar` example performs a slow RFID test, which shows the success or failure of each read or write
tag operation.

```
       ! U1 setvar "rfid.tag.test" "slow"

```

1537


SGD RFID Commands