# internal_wired.ip.wins.permanent_source



Specifies if the WINS address will be permanent or set via DHCP.


**Setvar**


To specify if the WINS address will be permanent or set via DHCP:

```
       ! U1 setvar "internal_wired.ip.wins.permanent_source" "value"

```

**Values**

              - `"off"` use DHCP-assigned WINS address

              - `"on"` use currently stored WINS address

The WINS address can be set using the `interface.network.active.wins_address`
command.


**Default**
```
          "off"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "internal_wired.ip.wins.permanent_source"

```

1200




SGD Network Commands