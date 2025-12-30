# wlan.ip.netmask



This setting refers to the wireless print server’s subnet mask address. This value is ignored if the IP
protocol is not set to permanent.


**Setvar**


To change the wireless print servers subnet mask:

```
       ! U1 setvar "wlan.ip.netmask" "value"

```

**Values**


Any valid subnet mask.


**Default**
```
          "255.255.255.0"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.ip.netmask"

```

**Example**

This `setvar` example shows the value set to `"255.255.255.0"` .

```
       ! U1 setvar "wlan.ip.netmask" "255.255.255.0"

```

When the `setvar` value is set to `"255.255.255.0"`, the `getvar` result is `"255.255.255.0"`


1428


SGD Network Commands