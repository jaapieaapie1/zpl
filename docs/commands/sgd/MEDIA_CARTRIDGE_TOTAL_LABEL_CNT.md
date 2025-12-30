# media.cartridge.total_label_cnt



This command returns the total number of labels that is initially available in the cartridge.


**Getvar**


To return the total number of labels initially available in the cartridge:

```
       ! U1 getvar "media.cartridge.total_label_cnt"

```

**Result**


The value depends on the length of the label and other factors. It typically ranges from 100-300
labels.

`"0"` inidciates that the cartridge is not installed

**Example**


In this example, the getvar returns with the total label count available in the cartridge.

```
          ! U1 getvar "media.cartridge.total_label_cnt" "100"

```

851


SGD Printer Commands