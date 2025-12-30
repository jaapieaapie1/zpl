# wlan.ip.dhcp.cid_enable



This command determines if DHCP (option 61) is turned on or off of the wireless print server.


**Setvar**


To instruct the printer to set the status of the client identifier of the wireless print server:

```
       ! U1 setvar "wlan.ip.dhcp.cid_enable" "value"

```

**Values**

              - `"off"` means the client identifier is turned off

              - `"on"` means the client identifier is turned on

**Default**
```
          "off"

```

**Getvar**


To instruct the printer to respond with the status of the client identifier of the wireless print server:

```
       ! U1 getvar "wlan.ip.dhcp.cid_enable"

```

**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "wlan.ip.dhcp.cid_enable" "off"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"off"` .


1409


SGD Network Commands