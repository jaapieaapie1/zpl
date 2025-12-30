# log.reboot.codes



Causes the printer to return a list of one-character values which indicates the reasons for the last 32 printer
reboots.


**Getvar**


To return with the reboot codes:

```
       ! U1 getvar "log.reboot.codes"

```

**Result**


A string of one-character codes indicating the reason for the reboots. A total of 32 reboot events
are stored; if less than 32 reboots have occurred, `"f"` is stored in any unpopulated event slot,
indicating "no data" for that event.


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
       ! U1 getvar "log.reboot.codes"
       "bb338bbbbbbbb3bbbbbbbbbbbbbbb1bb"

```

838


SGD Printer Commands