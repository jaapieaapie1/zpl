# ip.dhcp.cache_ip



This command enables or disables the IP caching.


**Setvar**


To set the status of the IP cache:

```
       ! U1 setvar "ip.dhcp.cache_ip" "value"

```

**Values**

              - `"on"` enabled

              - `"off"` disabled

**Default**
```
          "off"

```

**Getvar**


To retrieve the status of the IP cache:

```
       ! U1 getvar "ip.dhcp.cache_ip"

```

**Example**

This `setvar` example shows the value set to `"off”` .

```
       ! U1 setvar "ip.dhcp.cache_ip" "off”

```

When the `setvar` value is set to `"off"`, the `getvar` result is `"off"` .


1222


SGD Network Commands