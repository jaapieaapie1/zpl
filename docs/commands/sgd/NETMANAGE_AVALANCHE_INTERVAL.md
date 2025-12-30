# netmanage.avalanche.interval



This parameter obtains or sets the Network Management Update Interval time stored in the printer. Time is
measured in milliseconds (e.g., a setting of "2000" equals 2 seconds).


**Setvar**


To set the Network Management Update Interval:

```
       ! U1 setvar "netmanage.avalanche.interval" "value"

```

**Values**

Any integer value from `"0"` to `"4294967295"` (4,294,967,295 milliseconds)

**Default**
```
          "0"

```

**Getvar**


To retrieve the current Network Management Update Interval:

```
       ! U1 getvar "netmanage.avalanche.interval"

```

**Example**


This example sets the interval value to 3 seconds.

```
       ! U1 setvar "netmanage.avalanche.interval" "3000"

```

903


SGD Printer Commands