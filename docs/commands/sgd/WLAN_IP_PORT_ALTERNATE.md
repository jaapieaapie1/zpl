# wlan.ip.port_alternate



This command sets the port number of the alternate wlan port.


**NOTE:** Print servers supporting this command will monitor both the primary port and the alternate
port for connections at the same time.


**Setvar**


To set the alternate wlan port for the print server:

```
       ! U1 setvar "wlan.ip.port_alternate" "6101"

```

**Values**


Any valid TCP port address


**Default**
```
          "9100"

```

**Getvar**


To return the current alternate wlan port setting:

```
       ! U1 getvar "wlan.ip.port_alternate"

```

**Values**


The current port setting.


**Example 1**

This `setvar` example shows the value set to `"6101"` .

```
       U1 setvar "wlan.ip.port_alternate" "6101"

```

**Example 2**


This setvar example sets the channel mask to use only channels 1,6,11.

```
       ! U1 setvar "wlan.channel_mask" "0x421"

```

Only channels 1, 6, and 11 will be used by the radio.


1430




SGD Network Commands