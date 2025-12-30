# device.applicator.data_ready



This command will specify if a `"high"` or `"low"` value is required for the applicator to indicate it is ready
to receive data.


**Setvar**


To set the value:

```
       ! U1 setvar "device.applicator.data_ready" "value"

```

**Values**

              - `"high"`

              - `"low"`

**Default**

              - `"high"`

              - `"low"`


**Getvar**


To instruct the printer to respond with the currently set value:

```
       ! U1 getvar "device.applicator.data_ready"

```

664


SGD Printer Commands