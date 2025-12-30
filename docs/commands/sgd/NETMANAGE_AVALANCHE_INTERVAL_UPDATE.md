# netmanage.avalanche.interval_update



This parameter turns on or off the Network Management Update Interval.


This command is related to netmanage.avalanche.interval on page 903.


**Setvar**


To turn on or off the network management interval update:

```
       ! U1 setvar "netmanage.avalanche.interval_update" "value"

```

**Values**
```
          "off"
          "on"
```

**Default**
```
          "off"

```

**Getvar**


To retrieve the current network management interval update setting:

```
       ! U1 getvar "netmanage.avalanche.interval_update"

```

**Example**

This example sets the device’s Network Management Interval Update setting to `"on"` .

```
       ! U1 setvar "netmanage.avalanche.interval_update" "on"

```

904


SGD Printer Commands