# internal_wired.ip.timeout.enable



This network setting refers to enabling the connection timeout on the internal wired print server. For this to
take effect, the print server must be reset.


**Setvar**


To instruct the printer to enable or disable the timeout checking on the internal wired print server:

```
       ! U1 setvar "internal_wired.ip.timeout.enable" "value"

```

**Values**

              - `"off"` turns off the connection checking

              - `"on"` turns on the connection checking

**Default**
```
          "on"

```

**Getvar**


To instruct the printer to return whether the timeout checking is enabled on the internal wired print server:

```
       ! U1 getvar "internal_wired.ip.timeout.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "internal_wired.ip.timeout.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1197


SGD Network Commands