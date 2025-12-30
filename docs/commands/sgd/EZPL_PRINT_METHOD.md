# ezpl.print_method



This command sets the print method. This command is similar to `^MT.`


**Setvar**


To set the print method:

```
       ! U1 setvar "ezpl.print_method" "value"

```

**Values**

              - `"thermal trans"`

              - `"direct thermal"`

**Default**
```
          "thermal trans"

```

**Getvar**


To retrieve the current print method setting:

```
       ! U1 getvar "ezpl.print_method"

```

**Example**

This `setvar` example sets the print method to `"direct thermal"` .

```
       ! U1 "setvar ezpl.print_method" "direct thermal"

```

812


SGD Printer Commands