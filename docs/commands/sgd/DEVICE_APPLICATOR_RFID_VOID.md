# device.applicator.rfid_void



This command will specify if a `"high"` or `"low"` value is used for the RFID void signal, which occurs when
an RFID label is voided by the printer.


This command is supported only on the ZT411/ZT421 and ZT600 Series printers.


**Setvar**


To set the value:

```
       ! U1 setvar "device.applicator.rfid_void" "value"

```

**Values**

          - `"high"`

          - `"low"`


**Default**
```
       "low"

```

**Getvar**


To instruct the printer to respond with the currently set value:

```
       ! U1 getvar "device.applicator.rfid_void"

```

670




SGD Printer Commands