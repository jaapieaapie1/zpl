# wlan.preamble



This printer setting selects the radio preamble length to be used.


**Setvar**


This command instructs the printer to set the preamble length.

```
       ! U1 setvar "wlan.preamble" "value"

```

**Values**

`"long"` enables long preamble

`"short"` enables short preamble

**Default**
```
          "long"

```

**Getvar**


This command instructs the printer to respond with the current preamble length.

```
       ! U1 getvar "wlan.preamble"

```

**Example**

This `setvar` example shows the value set to `"long"` .

```
       ! U1 setvar "wlan.preamble" "long"

```

When the `setvar` value is set to `"long"`, the `getvar` result is `"long"` .


1466


SGD Network Commands