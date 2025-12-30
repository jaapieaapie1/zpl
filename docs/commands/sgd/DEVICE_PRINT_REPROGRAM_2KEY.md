# device.print_reprogram_2key



This command determines whether the printer will print a configuration label or 2key report after the
printer restarts following a firmware update.

When set to `"off"` the printer will not print the configuration label or 2key report after the printer is
updated.


**Setvar**


To set whether a two-key report is printed or not:

```
       ! U1 setvar "device.print_reprogram_2key" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**


To retrieve the current setting for processing two-key report after printer firmware is reprogrammed:

```
       ! U1 getvar "device.print_reprogram_2key"

```

**Example**


This example disables printing of the two-key report after printer firmware is reprogrammed.

```
       ! U1 setvar "device.print_reprogram_2key" "off"

```

733


SGD Printer Commands