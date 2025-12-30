# print.tone




SGD Printer Commands


This command specifies the printer darkness.


**Setvar**


To set the darkness and relative darkness:

```
! U1 setvar "print.tone" "value"

```

**Values**

    - `"0.0"` to `"30.0"` to adjust darkness

    - `"-0.1"` to `"-30.0"` and `"+0.1"` to `"+30.0"` for incremental adjustments

**Default**
```
   "4.0"

```

**Getvar**


To retrieve the printer’s current darkness setting:

```
! U1 getvar "print.tone"

```

**Example**

This `setvar` example sets the value to `"4.0"` .

```
! U1 setvar "print.tone" "4.0"

```

When the `setvar` value is set to `"4.0"`, the `getvar` result is `"4.0"` .


962


SGD Printer Commands