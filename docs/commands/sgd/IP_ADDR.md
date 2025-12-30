# ip.addr




SGD Network Commands


This command allows you to get or set the printer’s IP address.


**IMPORTANT:** For a set IP address to take affect, the IP protocol must be set to permanent and
the print server must be reset.


**Setvar**


To instruct the printer to change its current IP address upon powering the printer on:

```
! U1 setvar "ip.addr" "value"

```

**Values**


Any valid IP address


**Default**
```
   "0.0.0.0"

```

**Getvar**


To instruct the printer to respond with its current IP address:

```
! U1 getvar "ip.addr"

```

**NOTE:** The `setvar` value of this command can be affected by the `ip.dhcp.enable`
command.


**Example**

This `setvar` example shows the value set to `"10.14.4.235"` .

```
! U1 setvar "ip.addr" "10.14.4.235"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is
`"10.14.4.235"` .


1216


SGD Network Commands