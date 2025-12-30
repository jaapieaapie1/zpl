# ip.dhcp.arp_verify



This command configures the print server to send out an ARP request during the DHCP address
negotiation. This is used to verify if the IP address received from the DHCP server is in use.


          - If an ARP reply is received, a DHCP DECLINE message is sent to the DHCP server telling it that the
received address cannot be used and then the normal DHCP procedure is restarted.


          - If no ARP reply is received the DHCP address is used.


**Setvar**


To instruct the printer to turn on ARP verify:

```
       ! U1 setvar "ip.dhcp.arp_verify" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**


To return whether the printer will send the ARP request during the DHCP negotiation:

```
       ! U1 getvar "ip.dhcp.arp_verify"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.dhcp.arp_verify" "on"

```

1220




SGD Network Commands