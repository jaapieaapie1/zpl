# log.reboot.report



Causes the printer to return a list of values which indicate the reasons for the last 32 printer reboots.


**Getvar**


To return a list of values which indicate the reasons for the last 32 printer reboots:

```
       ! U1 getvar "log.reboot.report"

```

**Result**


The recorded reboot reasons as a list, starting with the most recent reboot reason first. A total of
32 reboot events are stored; if less than 32 reboots have occurred, "f" is stored in any unpopulated
event slot, indicating "no data" for that event.


**Values**
```
          "Other" "device.reset command" "Mirror reset – new files" "DTR off"
          "Low-battery timeout" "Low-battery shutdown" "power.shutdown command"
          "Idle timeout" "New OS reprogramming" "Unknown-1" "Unknown-2" "Off key"
          "No data"

```

**Example**

```
       ! U1 getvar "log.reboot.report"

```

A list of 32 codes, in a carriage-return delimited list:

```
       "Off key
       Off key
       Off key
       .
       .
       .
       No data
       "

```

840




SGD Printer Commands