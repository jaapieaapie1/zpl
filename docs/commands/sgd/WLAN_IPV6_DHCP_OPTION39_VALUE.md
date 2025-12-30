# wlan.ipv6.dhcp.option39_value



This command retrieves the value to be used for option 39 in DHCPv6 after processing the
`internal_wired.ipv6.dhcp.option39_format` command with a WLAN connection. The value is a
string of up to 127 characters.


**Getvar**


To retrieve the option 39 value:

```
       ! U1 getvar "wlan.ipv6.dhcp.option39_value"

```

1445


SGD Network Commands