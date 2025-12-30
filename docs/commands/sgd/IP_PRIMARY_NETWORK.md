# ip.primary_network



This command allows you to set the primary network to either wired or wireless.


**Setvar**


To set the primary network device:

```
       ! U1 setvar "ip.primary_network" "value"

```

**Values**
`"1"` means wired

`"2"` means wireless

**Default**
```
          "1"

```

**Getvar**


To respond with the name of the current primary network device:

```
       ! U1 getvar "ip.primary_network"

```

**Example**

This `setvar` example shows the value set to `"1"` .

```
       ! U1 setvar "ip.primary_network" "1"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"1"` .


1309


SGD Network Commands