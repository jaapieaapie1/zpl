# internal_wired.ip.addr



This command allows you to get or set the internal wired IP address of the print server.


**NOTE:** For a set IP address to take affect, the IP protocol must be set to permanent and the print
server must be reset.


**Setvar**


To instruct the printer to change its current internal wired print server IP address upon powering the printer
on:

```
       ! U1 setvar "internal_wired.ip.addr" "value"

```

**Values**


Any valid IP address


**Default**
```
          "0.0.0.0"

```

**Getvar**


To respond with the current internal wired print server IP address:

```
       ! U1 getvar "internal_wired.ip.addr"

```

**NOTE:** The `setvar` value of this command can be affected by the
`internal_wired.ip.dhcp.enable` command.


**Example**

This `setvar` example shows the value set to `"10.14.4.235"` .

```
       ! U1 setvar "internal_wired.ip.addr" "10.14.4.235"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is
`"10.14.4.235"` .


1169


SGD Network Commands