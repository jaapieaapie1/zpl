# cutter.clean_cutter



This command determines if the clean cutter option is enabled or disabled.


**Setvar**


To instruct the printer to set the clean cutter option:

```
       ! U1 setvar "cutter.clean_cutter"

```

**Values**

              - `"on"` turns on clean cutter

              - `"off"` turns off clean cutter

**Default**
```
          "on"

```

**Getvar**


To retrieve the status of the clean cutter option:

```
       ! U1 getvar "cutter.clean_cutter"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "cutter.clean_cutter" "on"

```

When the setvar value is set to `"on"`, the `getvar` result is `"on"` .


659


SGD Printer Commands