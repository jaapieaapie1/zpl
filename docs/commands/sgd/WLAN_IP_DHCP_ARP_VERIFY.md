# wlan.ip.dhcp.arp_verify



Specifies whether an ARP broadcast will be sent out to verify that the IP address received from the DHCP
server is not already in use.


**Setvar**


To set the ARP broadcast feature:

```
       ! U1 setvar "wlan.ip.dhcp.arp_verify" "value"

```

**Values**

              - `"on"` means an ARP broadcast will be sent out

              - `"off"` means an ARP broadcast will not be sent out


**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.ip.dhcp.arp_verify"

```

1406


SGD Network Commands