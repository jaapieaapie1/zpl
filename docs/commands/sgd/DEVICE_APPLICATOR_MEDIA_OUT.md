# device.applicator.media_out



This command will specify if a `"high"` or `"low"` value is required for an applicator to indicate that the
media has run out.


**Setvar**


To set the value:

```
       ! U1 setvar "device.applicator.media_out" "value"

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
       ! U1 getvar "device.applicator.media_out"

```

667


SGD Printer Commands