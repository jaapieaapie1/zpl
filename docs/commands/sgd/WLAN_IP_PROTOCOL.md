# wlan.ip.protocol



This command configures the IP addressing method used by the wireless print server.


**Setvar**


To configure the IP addressing method used by the wireless print server:

```
       ! U1 setvar "wlan.ip.protocol" "value"

```

**Values**

              - `"bootp"` uses the standard BOOTP addressing method to obtain an IP address and
configuration

              - `"dhcp"` uses the standard DHCP addressing method to obtain an IP address and configuration
for a server specified period of time

              - `"rarp"` uses the standard RARP addressing method to obtain an IP address

              - `"glean only"` uses the IP address from a PING packet that is sent to its hardware address
(unicast address)

              - `“permanent"` uses static values assigned through other commands

              - `"all"` tries all of the dynamic addressing methods, not permanent, to obtain an IP address

**Default**
```
          "all"

```

**Getvar**


To return the value of the currently selected IP protocol used by the wireless print server:

```
       ! U1 getvar "wlan.ip.protocol"

```

**Example**

In this example, the `setvar` result is the current programming language that the printer is using.

```
       ! U1 setvar "wlan.ip.protocol" "bootp"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"bootp"` .


1432


SGD Network Commands