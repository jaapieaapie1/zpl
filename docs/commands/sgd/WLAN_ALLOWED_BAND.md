# wlan.allowed_band



This command provides a method to restrict the 802.11n radio to either the 2.4 GHz or 5 GHz band.


Use this option when it is beneficial to have the radio use only one frequency band. Setting the 802.11n
radio to one band will reduce roaming and radio association times since the radio will not scan as many
channels.


**Setvar**


To restrict the 802.11n radio to either the 2.4 GHz or 5 GHz band:

```
       ! U1 setvar "wlan.allowed_band" "value"

```

**Values**

`"2.4"`, `"5"`,or `"all"`

**Default**
```
          "all"
```

If both bands are desired, use `"all"`


**Getvar**


To return the current setting for allowed bands:

```
       ! U1 getvar "wlan.allowed_band"

```

**Example**


This example sets the allowed band to only the 2.4 GHz band.

```
       ! U1 setvar "wlan.allowed_band" "2.4"

```

1385


SGD Network Commands