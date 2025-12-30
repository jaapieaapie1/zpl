# device.uptime



This command identifies the amount of time the printer has been powered on. The string format is: xx days,
xx hours, xx minutes, and xx seconds.


**Getvar**


To retrieve the amount of time the print has been powered on:

```
       ! U1 getvar "device.uptime"

```

**Example**

In this example, the `getvar` retrieves the amount of time the printer has been turned on.

```
       ! U1 getvar "device.uptime"
       "00 days 02 hours 45 mins 30 secs"

```

783


SGD Printer Commands