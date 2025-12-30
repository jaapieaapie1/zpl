# ezpl.label_length_max



This command sets the maximum label length in inches. This command is equivalent to the `^ML` ZPL
command.


**Setvar**


To set the maximum label length in inches:

```
       ! U1 setvar "ezpl.label_length_max" "value"

```

**Values**
```
          1.0 to 39.0
```

**Default**
```
          "39"

```

**Getvar**


To retrieve the current maximum label length setting in inches:

```
       ! U1 getvar "ezpl.label_length_max"

```

**Example**


This example sets the label length to 6.2 inches.

```
       ! U1 "setvar ezpl.label_length_max" "6.2"
       ! U1 "setvar ezpl.label_length_max" "14"

```

Values can be expressed to one decimal place.


807


SGD Printer Commands