# ip.http.enable



This printer setting refers to the HTTP protocol/web server setting.


**Setvar**


To change HTTP to on or off:

```
       ! U1 setvar "ip.http.enable" "value"

```

**Values**

`"off"` disables HTTP protocol

`"on"` enables HTTP protocol

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To respond with the HTTP status:

```
       ! U1 getvar "ip.http.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.http.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1259


SGD Network Commands