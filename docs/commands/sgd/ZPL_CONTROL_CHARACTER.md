# zpl.control_character



This command is used to change the control command prefix. The default prefix is the tilde (~).

This command is equivalent to the `^CT` and `~CT` commands.


**Setvar**


To change the control character:

```
       ! U1 setvar "zpl.control_character" "value"

```

**Values**

ASCII values for the desired character `"00-FF,00-ff,up,down"`

See ASCII on page 1560 for more information.


**Default**
```
          "7E" (tilde)

```

**Getvar**


To return the currently set control character:

```
       ! U1 getvar "zpl.control_character"

```

**Example**

This example sets the value set to `"+"` .

```
       ! U1 setvar "zpl.control_characater" "2b"

```

1072


SGD Printer Commands