# ip.dhcp.ntp.enable



This command controls whether or not the printer retrieves the address of a Network Time Protocol (NTP)
server during DHCP address assignment.


**Setvar**


To set whether or not the printer retrieves the address of a Network Time Protocol (NTP) server during
DHCP address assignment:

```
       ! U1 setvar "ip.dhcp.ntp.enable" "value"

```

**Values**

              - `"off"` does not request the NTP server address

              - `"on"` requests the NTP server address

**Default**
```
          "off"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "ip.dhcp.ntp.enable"

```

1234


SGD Network Commands