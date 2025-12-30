# print.troubleshooting_label_print



Sets whether batch counters will be displayed on the printer’s control panel.


**Setvar**


To set whether batch counters will be displayed on the printer’s control panel:

```
       ! U1 setvar "print.troubleshooting_label_print" "value"

```

**Values**

              - `"enabled"` specifies that batch counters will be displayed

              - `"disabled"` specifies that batch counters will not be displayed

**Default**
```
          "disabled"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "print.troubleshooting_label_print"

```

**Result**
```
          "enabled"

```

963


SGD Printer Commands