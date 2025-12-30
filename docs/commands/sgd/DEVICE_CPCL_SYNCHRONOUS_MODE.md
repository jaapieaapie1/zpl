# device.cpcl_synchronous_mode



This command enables/disables CPCL synchronous mode. When the printer is in sync mode, parsing will
"lock" while printing is going on, allowing behavior similar to that of the legacy SH3 mobile printers.


**Setvar**

To turn on or off the `device.cpcl_synchronous_mode` :

```
       ! U1 setvar "device.cpcl_synchronous_mode" "value"

```

**Values**

              - `"on"` puts the printer in CPCL synchronous mode

              - `"off"` puts the printer in default mode

**Default**
```
          "off"

```

**Getvar**


To return the current value of the setting:

```
       ! U1 getvar "device.cpcl_synchronous_mode"

```

**Example**

Consider issuing a label immediately followed by an SGD request. When sync mode is `"off"`, the SGD
will be returned nearly immediately after submitting the label. When sync mode is `"on"`, the SGD will be
returned after the label has printed.

```
       ! U1 setvar "device.cpcl_synchronous_mode" "on"

       ! U1 setvar "device.cpcl_synchronous_mode" "off"

```

682


SGD Printer Commands