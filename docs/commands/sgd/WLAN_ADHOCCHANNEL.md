# wlan.adhocchannel



This printer setting refers to specifying the wireless channel for adhoc channel.


**Setvar**


To set the printer’s wireless channel for adhoc channel mode:

```
       ! U1 setvar "wlan.adhocchannel" "value"

```

**Values**


Decimal value between 1 and 16 inclusive


**Default**
```
          "1"

```

**Getvar**


To respond with the printer’s wireless channel for adhoc channel mode:

```
       ! U1 getvar "wlan.adhocchannel"

```

**Example**

This `setvar` example shows the value set to `"1"` .

```
       ! U1 setvar "wlan.adhocchannel" "1"

```

When the `setvar` value is set to `"1"`, the `getvar` result is `"1"` .


1387


SGD Network Commands