# display.batch_counter



Sets whether batch counters will be displayed on the printer’s control panel.


**Setvar**


To set whether batch counters will be displayed on the printer’s control panel:

```
       ! U1 setvar "display.batch_counter" "value"

```

**Values**

              - `"enabled"` indicates batch counters will be displayed

              - `"disabled"` indicates batch counters will not be displayed

**Default**
```
          "disabled"

```

**Example**

```
       ! U1 setvar "display.batch_counter" "enabled"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "display.batch_counter"

```

**Result**
```
          "enabled"

```

796


SGD Printer Commands