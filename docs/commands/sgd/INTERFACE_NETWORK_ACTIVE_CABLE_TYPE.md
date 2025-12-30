# interface.network.active.cable_type



This command returns the cable type of the active network, either 10/100BaseT, Wireless 802.11b/g, or
Wireless 802.11n.


**NOTE:** This command will only give a valid response once an IP address has been established.


**Getvar**


To retrieve the current cable type of the active network:

```
       ! U1 getvar "interface.network.active.cable_type"

```

**Values**

              - `"10/100BaseT"`

              - `"Wireless 802.11b/g"`

              - `"Wireless 802.11n"`

**Default**


NA


1136


SGD Network Commands