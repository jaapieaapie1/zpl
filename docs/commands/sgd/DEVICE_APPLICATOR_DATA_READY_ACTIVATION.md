# device.applicator.data_ready_activation



Sets whether the applicator port DATA READY signal is asserted for all formats, or only for printing formats.


**Setvar**


To set whether the applicator port DATA READY signal is asserted for all formats, or only for printing
formats:

```
       ! U1 setvar "device.applicator.data_ready_activation" "value"

```

**Values**

              - `"print"` indicates the data ready signal is activated on printing labels only.

              - `"format"` indicates the data ready signal is activated on all formats.

**Default**
```
          "format"

```

**Getvar**


To return the data ready activation value:

```
       ! U1 getvar "device.applicator.data_ready_activation"

```

**Result**
"print"


**Example**


The setvar example shows the data ready signal activated on printing labels only.

```
       ! U1 setvar "device.applicator.data_ready_activation" "print"

```

777


SGD Printer Commands