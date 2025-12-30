# wlan.station_name



This printer setting refers to the station name.


**Setvar**


To set the station name:

```
       ! U1 setvar "wlan.station_name" "value"

```

**Values**


A maximum of 32 alphanumeric characters


**Default**
```
          "ZEBRA"

```

**Getvar**


To respond with the station name value:

```
       ! U1 getvar "wlan.station_name"

```

**Example**

This `setvar` example shows the value set to `"ZEBRA"` .

```
       ! U1 setvar "wlan.station_name" "ZEBRA"

```

When the `setvar` value is set to `"ZEBRA"`, the `getvar` result is `"ZEBRA"` .


1490




SGD Network Commands