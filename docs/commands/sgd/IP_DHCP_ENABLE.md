# ip.dhcp.enable



This printer setting turns DHCP on or off. DHCP is a method for acquiring an IP address, netmask, and
gateway automatically on printer power-up. It requires a DHCP server on the local network.


**NOTE:** If you are using static IP addressing, the IP protocol must be set to permanent.


**Setvar**


To instruct the printer to turn DHCP on or off:

```
       ! U1 setvar "ip.dhcp.enable" "value"

```

**Values**

              - `"off"` printer does not use DHCP to get the IP address

              - `"on"` printer uses DHCP to get the IP address

**Default**
```
          "on"

```

**Getvar**


To instruct the printer to respond with the DHCP status:

```
       ! U1 getvar "ip.dhcp.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.dhcp.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1230




SGD Network Commands