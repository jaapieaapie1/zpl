# device.xml.enable



This command enables and disables language parsing support for XML. When enabled (on), the printer will
parse both ZPL and XML. When disabled (off), the printer will not parse XML data.


**Setvar**


To instruct the printer to disable or enable the language parsing support for XML:

```
       ! U1 getvar "device.xml.enable" "value"

```

**Values**

              - `"on"` enables language parsing support for XML

              - `"off"` disables language parsing support for XML

**Default**
```
          "on"

```

**Getvar**


To enable and disable language parsing support for XML:

```
       ! U1 getvar "device.xml.enable"

```

**Example**

This `setvar` example shows the language parsing support for XML set to `"on"` .

```
       ! U1 setvar "device.xml.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is language parsing support for XML set to
`"on"` .


789


SGD Printer Commands