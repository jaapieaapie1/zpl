# zpl.left_position



This command sets the label’s left margin offset in dots.

Related Command: `^LS`


**Setvar**


To set the label left margin offset in dots:

```
       ! U1 setvar "zpl.left_position" "value"

```

**Values**

`"-9999"` to `"9999"`

**Default**
```
          "0"

```

**Getvar**


To retrieve the currently set left margin offset for the label:

```
       ! U1 getvar "zpl.left_position"

```

**Example**

```
       ! U1 setvar "zpl.left_position" "100"

```

1077


SGD Printer Commands