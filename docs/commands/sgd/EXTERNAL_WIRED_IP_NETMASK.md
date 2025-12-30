# external_wired.ip.netmask



This setting refers to the external wired print server’s subnet mask address. This value is ignored if the IP
protocol is not set to permanent.


**Setvar**


To instruct the printer to change the external wired print servers subnet mask:

```
       ! U1 setvar "external_wired.ip.netmask" "value"

```

**Values**


Any valid subnet mask.


**Default**
```
          "255.255.255.0"

```

**Getvar**


To instruct the printer to respond with the external wired print server’s subnet mask:

```
       ! U1 getvar "external_wired.ip.netmask"

```

On SEH print server models PS102-Z or the PS105-Z, only the `getvar` command is supported.


**Example**

This `setvar` example shows the value set to `"255.255.255.0"` .

```
       ! U1 setvar "external_wired.ip.netmask" "255.255.255.0"

```

When the `setvar` value is set to `"255.255.255.0"`, the `getvar` result is `"255.255.255.0"` .


1125


SGD Network Commands