# internal_wired.ip.dhcp.cid_enable



This command determines if DHCP (option 61) is turned on or off of the internal wired print server.


**Setvar**


To instruct the printer to set the status of the client identifier of the internal wired print server:

```
       ! U1 setvar "internal_wired.ip.dhcp.cid_enable" "value"

```

**Values**

              - `"off"` client identifier is turned off

              - `"on"` client identifier is turned on

**Default**
```
          "off"

```

**Getvar**


To instruct the printer to respond with the status of the client identifier of the internal wired print server:

```
       ! U1 getvar "internal_wired.ip.dhcp.cid_enable"

```

**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "internal_wired.ip.dhcp.cid_enable" "off"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"off"` .


1175


SGD Network Commands