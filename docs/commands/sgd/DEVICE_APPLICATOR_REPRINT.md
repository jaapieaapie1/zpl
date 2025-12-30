# device.applicator.reprint



This command will specify if a `"high"` or `"low"` value is required for an applicator to reprint a label.


**Setvar**


This command is similar to ~PR on page 324.


To set the value:

```
       ! U1 setvar "device.applicator.reprint" "value"

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
       ! U1 getvar "device.applicator.reprint"

```

669


SGD Printer Commands