# external_wired.ip.timeout.enable



This network setting refers to enabling the connection timeout on the external wired 10/100 print server.
For this to take effect, the print server must be reset.


**Setvar**


To instruct the printer to enable or disable the timeout checking on the external wired print server:

```
       ! U1 setvar "external_wired.ip.timeout.enable" "value"

```

**Values**

              - `"off"` turns off the connection checking

              - `"on"` turns on the connection checking

**Default**
```
          "on"

```

**Getvar**


To instruct the printer to return whether the timeout checking is enabled on the external wired print server:

```
       ! U1 getvar "external_wired.ip.timeout.enable"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "external_wired.ip.timeout.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1128


SGD Network Commands