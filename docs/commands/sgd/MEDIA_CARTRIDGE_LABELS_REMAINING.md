# media.cartridge.labels_remaining



This command returns the number of labels which remain in the cartridge.


**Getvar**


To return the number of remaining labels:

```
       ! U1 getvar "media.cartridge.labels_remaining"

```

**Result**


An integer >= 0.

`"no"` indicates that the cartridge is not inserted or the printer does not support this command.

**Example**

In this example, the `getvar` returns the number of print labels that is remaining in the cartridge.

```
          ! U1 getvar "media.cartridge.labels_remaining" "10"

```

849


SGD Printer Commands