# internal_wired.activity_led



Controls whether the Ethernet activity LED will blink or be solid when there is link.


**Setvar**


To set the Ethernet activity LED value:

```
       ! U1 setvar "internal_wired.activity_led" "value"

```

**Values**

              - `"blink"` means the LED will blink

              - `"solid"` means the LED will be solid

**Default**
```
          "blink"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "internal_wired.activity_led"

```

1165


SGD Network Commands