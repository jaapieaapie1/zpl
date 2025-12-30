# internal_wired.ipv6.dhcp.option39_format



This command sets the value of the format field for option 39 (to configure the Fully Qualified Domain
Name or FQDN) in DHCPv6 with a wired connection. The value is a string up to 127 characters, and the
default is the value set by `device.friendly_name` .


**Setvar**


To set the format string:

```
       ! U1 setvar "internal_wired.ipv6.dhcp.option39_format" "value"

```

where `"value"` is a string of up to 127 characters.

If the value is a source SGD command, it must be bracketed with `"<"` and `">"`, such as
`"<device.friendly_name>"` .


**Getvar**


To retrieve the string:

```
       ! U1 getvar "internal_wired.ipv6.dhcp.option39_format"

```

**Example**

This command sets the value to `"printer2"` .

```
       ! U1 setvar "internal_wired.ipv6.dhcp.option39_format" "printer2"

       ! U1 getvar "internal_wired.ipv6.dhcp.option39_format"

```

**Result**

```
          "printer2"

```

1207


SGD Network Commands