# parallel_port.mode



This command sets the mode type for the parallel port.


**Setvar**


To set the mode type for the parallel port:

```
       ! U1 setvar "parallel_port.mode" "value"

```

**Values**

              - `"bidirectional"`

              - `"unidirectional"`

**Default**
```
          "bidirectional"

```

**Getvar**


To retrieve the current mode type setting for the parallel port:

```
       ! U1 getvar "parallel_port.mode"

```

**Example**

```
       ! U1 setvar "parallel_port.mode" "bidirectional"

```

938


SGD Printer Commands