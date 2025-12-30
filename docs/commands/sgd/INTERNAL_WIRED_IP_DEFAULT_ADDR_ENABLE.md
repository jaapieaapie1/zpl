# internal_wired.ip.default_addr_enable



This command allows you to default the internal wired print server’s IP address.


**IMPORTANT:** For a set IP address to take affect, the IP protocol must be set to permanent and
the print server must be reset.


**Setvar**


To tell the printer to use it’s default address:

```
       ! U1 setvar "internal_wired.ip.default_addr_enable" "value"

```

**Values**

              - `"on"` means enabled

              - `"off"` means disabled

**Default**
```
          "on"

```

**Getvar**


To instruct the printer to show the status of the setting of internal wired print server’s default IP address
feature:

```
       ! U1 getvar "internal_wired.ip.default_addr_enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "internal_wired.ip.default_addr_enable" "on"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"on"` .


1171


SGD Network Commands