# media.media_low.warning



This command retrievs the value of, or enables or disables the Supplies Warning system.


**Setvar**


To enable or disable the Supplies Warning System:

```
       ! U1 setvar "media.media_low.warning" "value"

```

**Values**

`"disabled"` indicates not active

`"enabled"` indicates active

**Default**
```
          "disabled"

```

**Getvar**


To retrieve the setting for the Supplies Warning system:

```
       ! U1 getvar "media.media_low.warning"

```

**Example**

This `setvar` example disables the Supplies Warning system.

```
       ! U1 setvar "media.media_low.warning" "disabled"

```

864


SGD Printer Commands