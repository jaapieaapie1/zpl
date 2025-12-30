# internal_wired.ip.dhcp.arp_verify



This command controls whether the internal wired print server will send an ARP response after receiving
an IP address via DHCP.


**Setvar**


To control whether the internal wired print server will send an ARP response after receiving an IP address
via DHCP:

```
       ! U1 setvar "internal_wired.ip.dhcp.arp_verify" "value"

```

**Values**

`"off"` ARP response will not be sent.

`"on"` ARP response will be sent.

**Default**

`"off"` for mobile Link-OS printers

`"on"` for desktop and industrial printers


**Getvar**


To return the current setting value:

```
       ! U1 getvar "internal_wired.ip.dhcp.arp_verify"

```

1172


SGD Network Commands