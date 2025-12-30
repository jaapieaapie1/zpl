# media.feed_skip



This command only applies to labels created with CPCL commands. It controls the same setting as the
second parameter of the CPCL SETFF command.


**Setvar**


To set the printer's feed skip length:

```
       ! U1 setvar "media.feed_skip" "value"

```

**Values**

A numeric value from `"0"` to `"50"` .

**Default**
```
          "5"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "media.feed_skip"

```

**Result**

A numeric value from `"0"` to `"50"` .


861


SGD Printer Commands