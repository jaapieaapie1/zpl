# external_wired.ip.default_addr_enable



This command allows you to default the external wired print server’s IP address.


**IMPORTANT:** For a set IP address to take affect, the IP protocol must be set to permanent and
the print server must be reset.


**Setvar**


To instruct the printer to use it’s default address:

```
       ! U1 setvar "external_wired.ip.default_addr_enable" "value"

```

If no address is provided through DHCP or BOOTP. If you do not assign an IP address after 2 minutes, the
10/100 Internal PS defaults to IP address 192.168.254.254.


**Values**

              - `"on"` enabled

              - `"off"` disabled

**Default**
```
          "on"

```

**Getvar**


To instruct the printer to show the status of the setting of external wired print server’s default IP address
feature:

```
       ! U1 getvar "external_wired.ip.default_addr_enable"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "external_wired.ip.default_addr_enable" "on"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"on"` .


1118


SGD Network Commands