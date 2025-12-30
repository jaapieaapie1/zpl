# zpl.format_prefix



This command sets or returns the format character to the character corresponding to the ASCII Hex value
xx. This command is similar to the `^CC` and `~CC` commands.


**Setvar**


To set the format character to the character corresponding to the ASCII Hex value xx:

```
       ! U1 setvar "zpl.format_prefix" "xx"

```

**Values**

`"xx"` = `"00-FF"`, `"00-ff"`, `"up"`, `"down"`

              - Specifying the up or down will increment or decrement the current value.


              - Note that the increment/decrement will skip over conflicting values.

              - Conflicting values for `zpl.format_prefix` are `zpl.command_prefix`, and
`zpl.delimiter` .


**Getvar**


To return the current value of the format character:

```
       ! U1 getvar "zpl.format_prefix"

```

**Result**
```
          ^ (5E)

```

1070