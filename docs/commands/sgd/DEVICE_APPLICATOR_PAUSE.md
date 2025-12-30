# device.applicator.pause



This command will specify if a `"high"` or `"low"` value is required for an applicator to pause printing.


**Setvar**


To set the value:

```
       ! U1 setvar "device.applicator.pause" "value"

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
       ! U1 getvar "device.applicator.pause"

```

668


SGD Printer Commands