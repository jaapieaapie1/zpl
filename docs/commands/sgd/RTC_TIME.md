# rtc.time




SGD Printer Commands


This command sets the system time. The time value must be in the hour:minute:second format, for
example, 11:32:11. If the format is not correct or the time is not valid, the command is ignored. The printer
needs to be reset using `! U1 do "device.reset" ""` to accept this command.


**Getvar**


To retrieve the current system time:

```
! U1 getvar "rtc.time"

```

**Value**
Returns the current system time in the hour:minute:second format.


**Setvar**


To set the current system time:

```
! U1 setvar "rtc.time" "value"

```

**Value**
Accepts a time in hour:minute:second format. If the format is not correct or the time is not valid, the
command is ignored.


**Example**

This `setvar` example shows the value set to 11:32 and 11 seconds (11:32:11).

```
! U1 setvar "rtc.time" "11:32:11"

```

The `setvar` value is the `getvar` result. In this example, the `getvar` result is `"11:32:11"` .


975


SGD Printer Commands