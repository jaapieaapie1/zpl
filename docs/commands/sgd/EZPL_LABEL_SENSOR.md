# ezpl.label_sensor



This command sets the paper out threshold value.


**Setvar**


To set the paper out threshold value:

```
       ! U1 setvar "ezpl.label_sensor" "value"

```

**Values**

`"0" to "255",` integer values only

**Default**
```
          "70"

```

**Getvar**


To retrieve the currently set paper out threshold value:

```
       ! U1 getvar "ezpl.label_sensor"

```

**Example**

This `setvar` example shows the value set to `50` .

```
       ! U1 setvar "ezpl.label_sensor" "50"

```

808


SGD Printer Commands