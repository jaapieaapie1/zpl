# device.friendly_name



This command shows the name assigned to the printer.


**Setvar**


To set the printer’s name:

```
       ! U1 setvar "device.friendly_name" "value"

```

**Default**

`"xxxxxxxxxx"` ( `"xxxxxxxxxx"` represents the main logic board serial number)


**Getvar**


To retrieve the name assigned to the printer:

```
       ! U1 getvar "device.friendly_name"

```

**Example**

This `setvar` example shows the value set to `"xxxxxxxxxx"` .

```
       ! U1 setvar "device.friendly_name" "xxxxxxxxxx"

```

When the `setvar` value is set to `"xxxxxxxxxx"`, the `getvar` result is `"xxxxxxxxxx"` .


696


SGD Printer Commands