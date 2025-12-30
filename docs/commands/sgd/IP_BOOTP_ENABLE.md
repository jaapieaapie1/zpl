# ip.bootp.enable



This printer setting turns BOOTP on or off. BOOTP is a method for acquiring an IP address, netmask, and
gateway automatically on printer power-up. It requires a BOOTP server on the local network.


**NOTE:** If you are using static IP addressing, the IP protocol must be set to permanent.


**Setvar**


To instruct the printer to turn BOOTP on or off:

```
       ! U1 setvar "ip.bootp.enable" "value"

```

**Values**

              - `"off"` printer does not use BOOTP to get the IP address

              - `"on"` printer uses BOOTP to get the IP address

**Default**
```
          "on"

```

**Getvar**


To instructs the printer to respond with the current BOOTP setting:

```
       ! U1 getvar "ip.bootp.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.bootp.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1219


SGD Network Commands