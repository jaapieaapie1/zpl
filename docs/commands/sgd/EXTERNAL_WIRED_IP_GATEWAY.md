# external_wired.ip.gateway



This command instructs the printer to change the external wired print server’s gateway address.


**IMPORTANT:** This setting refers to the gateway address. A set value is ignored if the IP protocol
is not set to permanent.


**Setvar**


To instruct the printer to change the external wired printer server’s gateway address:

```
       ! U1 setvar "external_wired.ip.gateway" "value"

```

**Values**


Any valid gateway address


**Default**
```
          "0.0.0.0"

```

**Getvar**


To instruct the printer to respond with the external wired printer server’s gateway address:

```
       ! U1 getvar "external_wired.ip.gateway"

```

On SEH print server models PS102-Z or the PS105-Z, only the `getvar` command is supported.


**Example**

This `setvar` example shows the value set to `"10.3.5.1"` .

```
       ! U1 setvar "external_wired.ip.gateway" "10.3.5.1"

```

When the `setvar` value is set to `"10.3.5.1"`, the `getvar` result is `"10.3.5.1"` .


1124


SGD Network Commands