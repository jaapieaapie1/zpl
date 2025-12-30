# ip.ipp.enable



This command sets or retrieves if the Internet Printing Protocol (IPP) feature is on. When you send this
command, a network reset is required for the change to take effect.


**Setvar**


To enable or disable the IPP feature:

```
       ! U1 setvar "ip.ipp.enable" "value"

```

**Values**

              - `"on"` enables IPP

              - `"off"` disables IPP

**Default**
```
          "on"

```

**Getvar**


To retrieve the current setting of the IPP feature:

```
       ! U1 getvar "ip.ipp.enable"

```

**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "ip.ipp.enable" "off"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"off"` .


1264


SGD Network Commands