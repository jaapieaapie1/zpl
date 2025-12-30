# wlan.ip.default_addr_enable



This command allows you to default the wireless print server’s IP address.


**IMPORTANT:** For a set IP address to take affect, the IP protocol must be set to permanent and
the print server must be reset.


**Setvar**


To enable or disable te printer to use its default IP address, if no address is provided through DHCP or
BOOTP:

```
       ! U1 setvar "wlan.ip.default_addr_enable" "value"

```

If you do not assign an IP address after 2 minutes, the 10/100 Internal PS defaults to IP address
192.168.254.254.


**Values**

              - `"on"` enabled

              - `"off"` disabled

**Default**
```
          "on"

```

**Getvar**


To instruct the printer to show the status of the setting of the wireless print server’s default IP address
feature:

```
       ! U1 getvar "wlan.ip.default_addr_enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "wlan.ip.default_addr_enable" "on"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"on"` .


1405


SGD Network Commands