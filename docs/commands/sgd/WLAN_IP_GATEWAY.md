# wlan.ip.gateway



This command instructs the printer to change the wireless print server’s gateway address.


**IMPORTANT:** This setting refers to the gateway address. A set value is ignored if the IP protocol
is not set to permanent.


**Setvar**


To change the wireless printer server’s gateway address:

```
       ! U1 setvar "wlan.ip.gateway" "value"

```

**Values**


Any valid gateway address


**Default**
```
          "0.0.0.0"

```

**Getvar**


To respond with the wireless printer server’s gateway address:

```
       ! U1 getvar "wlan.ip.gateway"

```

**Example**

This `setvar` example shows the value set to `"10.3.5.1"` .

```
       ! U1 setvar "wlan.ip.gateway" "10.3.5.1"

```

When the `setvar` value is set to `"10.3.5.1"`, the `getvar` result is `"10.3.5.1"` .


1427


SGD Network Commands