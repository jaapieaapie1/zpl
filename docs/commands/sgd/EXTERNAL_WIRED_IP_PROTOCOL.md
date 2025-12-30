# external_wired.ip.protocol



This command configures the IP addressing method used by the external wired print server.


**Setvar**


To instruct the printer to configure the IP addressing method used by the external wired print server:

```
       ! U1 setvar "external_wired.ip.protocol" "value"

```

**Values**

              - `"bootp"` uses the standard bootp addressing method to obtain an IP address and
configuration

              - `"dhcp"` uses the standard dhcp addressing method to obtain an IP address and configuration
for a server specified period of time

              - `"rarp"` uses the standard rarp addressing method to obtain an IP address

              - `"glean"` uses the IP address from a PING packet that is sent to its hardware address (unicast
address)

              - `"permanent"` uses static values assigned through other commands

              - `"all"` tries all of the dynamic addressing methods, not permanent, to obtain an IP address

**Default**
```
          "all"

```

**Getvar**


To return the IP addressing method used by the external print server:

```
       ! U1 getvar "external_wired.ip.protocol"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**Example**

In this example, the `setvar` result is the current programming language that the printer is using.

```
       ! U1 setvar "external_wired.ip.protocol" "bootp"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"bootp"` .


1127


SGD Network Commands