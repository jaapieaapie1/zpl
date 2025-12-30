# device.mcu_cutter.desired_revision



This command reports the revision number contained in the download application file for the Cutter MCU.
When the contents of this SGD can be used in conjunction with `device.mcu_cutter.revision` to
determine if the download of the Cutter MCU was successful.


**Getvar**


To retrieve the revision number of the most recent application file downloaded to the printer:

```
       ! U1 getvar "device.mcu_cutter.desired_revision"

```

**Values**
`"1"` to `"254"`


721


SGD Printer Commands