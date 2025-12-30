# internal_wired.ip.dhcp.cache_ip



This command enables or disables the IP cache on the internal wired print server.


**Setvar**


To set the status of the IP cache:

```
       ! U1 setvar "internal_wired.ip.dhcp.cache_ip" "value"

```

**Values**

              - `"on"` means enabled

              - `"off"` means disabled

**Default**
```
          "off"

```

**Getvar**


To retrieve the status of the IP cache on the internal wired print server:

```
       ! U1 getvar "internal_wired.ip.dhcp.cache_ip"

```

**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "internal_wired.ip.dhcp.cache_ip" "off"

```

When the `setvar` value is set to `"off"`, the `getvar` result is `"off"` .


1173


SGD Network Commands