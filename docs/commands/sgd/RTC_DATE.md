# rtc.date




SGD Printer Commands


The command sets the system date. The command accepts the month-day-year format, for example,
"11-02-2021". If the format is not correct or the date is not valid, the command is ignored. The printer needs
to be reset using `! U1 do "device.reset" ""` to accept this command.


**Getvar**


To report the current system date:

```
! U1 getvar "rtc.date"

```

**Value**


The command reports the system date in the month-day-year format.


**Setvar**


To set the system date:

```
! U1 setvar "rtc.date" "value"

```

**Value**
Accepts a date in the month-day-year format. If the format is not correct or the date is not valid, the
command is ignored.


**Example**

This `setvar` example shows the value set to November 2, 2021 (11-02-2021).

```
! U1 setvar "rtc.date" "11-02-2021"

```

The `setvar` value is the `getvar` result. In this example, the `getvar` result is `"11-02-2021"` .


974