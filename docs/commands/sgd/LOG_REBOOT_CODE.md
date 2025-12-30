# log.reboot.code



Causes the printer to return a one-character value which indicates the reason for the last printer reboot.


**Getvar**


To return a one-character value which indicates the reason for the last printer reboot:

```
       ! U1 getvar "log.reboot.code"

```

**Result**


A one-character code indicating the reason for the reboot.


**Values**

              - `"0"` Other

              - `"1"` device.reset command

              - `"2"` Mirror reset – new files

              - `"3"` DTR off

              - `"4"` Low-battery timeout

              - `"5"` Low-battery shutdown

              - `"6"` power.shutdown command

              - `"7"` Idle timeout

              - `"8"` Printer OS update

              - `"9"` Reserved

              - `"a"` Reserved

              - `"b"` Off key

              - `"f"` No data


**Example**

```
       ! U1 getvar "log.reboot.code"
       "4"

```

The result indicates that the device rebooted because the battery timed out.


837


SGD Printer Commands