# comm.halt



This command halts all communications to the serial port if an error condition occurs.


**Setvar**


To instruct the printer to halt communication to the printer:

```
       ! U1 setvar "comm.halt" "value"

```

**Values**

              - `"yes"`

              - `"no"`

**Default**
```
          "yes"

```

**Getvar**


To return the current value:

```
       ! U1 getvar "comm.halt"

```

**Values**

              - `"yes"`

              - `"no"`


**Example**

This `setvar` example sets the value set to `"yes"` .

```
       ! U1 setvar "comm.halt" "yes"

```

651


SGD Printer Commands