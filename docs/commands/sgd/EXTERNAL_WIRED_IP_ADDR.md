# external_wired.ip.addr



This command allows you to get or set the external wired print server’s IP address.


**IMPORTANT:** For a set IP address to take affect, the IP protocol must be set to permanent and
the print server must be reset.


**Setvar**


To instruct the printer to change its current external wired print server IP address upon powering the
printer on:

```
       ! U1 setvar "external_wired.ip.addr" "value"

```

**Values**


Any valid IP address


**Default**
```
          "0.0.0.0"

```

**Getvar**


To respond with the current external wired print server IP address:

```
       ! U1 getvar "external_wired.ip.addr"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**NOTE:** The `setvar` value of this command can be affected by the
`external_wired.ip.dhcp.enable` command.


**Example**

This `setvar` example shows the value set to `"10.14.4.235"` .

```
       ! U1 setvar "external_wired.ip.addr" "10.14.4.235"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is
`"10.14.4.235"` .


1116


SGD Network Commands