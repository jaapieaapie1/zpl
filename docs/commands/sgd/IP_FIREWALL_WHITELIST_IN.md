# ip.firewall.whitelist_in



Returns a comma-separated list of IP addresses and/or IP address ranges that will be allowed to
communicate with the printer. If the list is empty then the firewall will be disabled.


**Setvar**


To set the command:

```
       ! U1 setvar "ip.firewall.whitelist_in" "value"

```

**Value**


The value is a string of up to 256 alphanumeric characters.


**Default**

```
          ""

```

**Examples**


Single IP address:

```
       ! U1 setvar "ip.firewall.whitelist_in" "192.168.1.20"

```

Multiple IP addresses:

```
       ! U1 setvar "ip.firewall.whitelist_in" "192.168.1.20,192.168.100.21"

```

IP address ranges:

```
       ! U1 setvar "ip.firewall.whitelist_in" "192.168.1.20-192.168.1.100"

```

IP ranges and Single/Multiple IPs

```
       "ip.firewall.whitelist_in" "192.168.1.20-192.168.1.40, 192.168.1.50,
       192.168.1.75"

```

**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "ip.firewall.whitelist_in"

```

1250




SGD Network Commands