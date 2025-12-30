# display.language



This command sets the display language for the control panel. This is equivalent to the `^KL` ZPL command.


**Setvar**


To set the display language for the front panel:

```
       ! U1 setvar "display.language" "language"

```

**Values**

              - `"english"`

              - `"spanish"`

              - `"french"`

              - `"german"`

              - `"italian"`

              - `"norwegian"`

              - `"portuguese"`

              - `"swedish"`

              - `"danish"`

              - `"spanish2"` (same as Spanish)

              - `"dutch"`

              - `"finnish"`

              - `"japanese"`

              - `"korean"`

              - `"simplified chinese"`

              - `"traditional chinese"`

              - `"russian"`

              - `"polish"`

              - `"czech"`

              - `"romanian"`

**Default**
```
          "english"

```

**Getvar**


To return the currently set display language:

```
       ! U1 getvar "display.language"

```

799


SGD Printer Commands


**Example**

This `setvar` example shows the value set to `"dutch"` .

```
! U1 setvar "dsplay.language" "dutch"

```

800




SGD Printer Commands