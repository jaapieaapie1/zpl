# wlan.ip.timeout.value



This network setting refers to the number of seconds before the connection times out for the wireless print
server. For this to take effect, the print server must be reset.


**Setvar**


To set the time value of the wireless print server:

```
       ! U1 setvar "wlan.ip.timeout.value" "value"

```

**Values**

`"1"` through `"3600"`

**Default**
```
          "300"

```

**Getvar**


To respond with the current setting value:

```
       ! U1 getvar "wlan.ip.timeout.value"

```

**Example**

This `setvar` example shows the value set to `"300"` .

```
       ! U1 setvar "wlan.ip.timeout.value" "300"

```

When the `setvar` value is set to `"300"`, the `getvar` result is `"300"` .


1434


SGD Network Commands