# device.applicator.service_required



This command will specify if a `"high"` or `"low"` value is required for an applicator to indicate that
maintenance is required.


**Setvar**


To set the value:

```
       ! U1 setvar "device.applicator.service_required" "value"

```

**Values**

              - `"high"`

              - `"low"`

**Default**
```
          "low"

```

**Getvar**


To instruct the printer to respond with the currently set value:

```
       ! U1 getvar "device.applicator.service_required"

```

673


SGD Printer Commands