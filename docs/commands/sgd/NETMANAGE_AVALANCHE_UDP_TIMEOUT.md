# netmanage.avalanche.udp_timeout



This command sets the device’s Network Management UDP timeout. Time is set in milliseconds.


**Setvar**


To set the device’s Network Management UDP timeout:

```
       ! U1 setvar "netmanage.avalanche.udp_timeout" "value"

```

**Values**

Any integer value from `"0"` to `"4294967295"` (4,294,967,295 milliseconds)

**Default**
```
          "0"

```

**Getvar**


To return the current Network Management UDP timeout setting:

```
       ! U1 getvar "netmanage.avalanche.udp_timeout"

```

**Example**


This example sets the timeout value to .4 seconds (400 milliseconds).

```
       ! U1 setvar "netmanage.avalanche.udp_timeout" "400"

```

913


SGD Printer Commands