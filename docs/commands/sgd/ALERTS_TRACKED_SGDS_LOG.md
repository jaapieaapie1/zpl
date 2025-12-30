# alerts.tracked_sgds.log



This command reports the log of the settings listed in `alerts.tracked_settings.log_tracked` . The
log entries will be fully JSON complaint.


**Getvar**


To retrieve the current log:

```
       ! U1 getvar "alerts.tracked_sgds.log"

```

**Example**

Sending `! U1 getvar "alerts.tracked_settings.log"` returns:

```
       :"[{"settingsName":"newValue","timestamp"
       :"06-24-2012 19:51:28.641"}]" for 1 entry or
       "[{"settingsName":"newValue","timestamp"
       :"06-24-2012
       19:51:28.641"},\r\n{"settingsName2":"newValue2","timestamp":"06-24-2012
       19:51:30.641"}] for 2 entries.

```

When the log is empty, the result will be: `""`


628


SGD Printer Commands