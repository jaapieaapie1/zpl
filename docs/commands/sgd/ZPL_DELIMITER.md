# zpl.delimiter



This command is used to change the delimiter character for ZPL commands. The default delimiter
character is the comma (,).

Related Commands: `^CD` and `~CD` commands.


**Setvar**


To change the delimiter character for ZPL commands:

```
       ! U1 setvar "zpl.delimiter" "value"

```

**Values**

HEX values for the desired character `"00-FF,00-ff,up,down"`

See ASCII on page 1560 for more information.


**Default**
```
          "2C" (comma)

```

**Getvar**


To return the currently set delimiter character:

```
       ! U1 getvar "zpl.delimiter"

```

**Example**


This example changes the delimiter to a semi-colon (;).

```
       ! U1 setvar "zpl.delimiter" "3B"

```

1073


SGD Printer Commands