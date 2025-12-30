# media.extended_presentation



This command increases the peel presentation position to match the extended peel plate option on some
printer models.


**Setvar**


**NOTE:** This command is supported only by the ZT411 and ZT421 printers.


When this parameter is enabled, the peel-bar-to-printline, tear-off-to-printline, and applicator-to-printline
distances are increased.


When this parameter is disabled, the distances return to their initial values.


To set the extended media presentation:

```
       ! U1 setvar "media.extended_presentation" "value"

```

**Values**

`"enabled"` enables extending the peel presentation position

`"disabled"` disables extending the peel presentation position

**Default**
```
          "disabled"
```

The initial value is `"disabled"` ; however, this setting will not reset to a particular default.


**Getvar**


To retrieve the extended media presentation:

```
       ! U1 getvar "media.extended_presentation"

```

**Example**

This `setvar` example enables the command by setting the value to `"enabled"` .

```
       ! U1 setvar "media.extended_presentation" "enabled"

```

860




SGD Printer Commands