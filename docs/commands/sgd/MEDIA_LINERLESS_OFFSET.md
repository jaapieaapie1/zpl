# media.linerless_offset



This command sets or retrieves the value of the linerless media offset (or the size of the so-called "no
print zone" or "dead zone"). This offset, which is at the beginning of the label, starts at the top-of-form and
continues in the number of pixels defined by the last user-specified value.


**Setvar**


To set the printer to adjust the linerless media offset:

```
       ! U1 setvar "media.linerless_offset" "value"

```

**Values and Defaults**


For the ZT411 linerless printers, the no print zone or leader length data range and default value are
DPI dependent:

|DPI|Min|Default|Max|
|---|---|---|---|
|203|0|61|76|
|300|0|90|113|
|600|0|180|225|



For other printers with a legacy cutter installed, the default value is `"0"`, and the range is `"0"` to
`"225"` .


**Getvar**


To retrieve the existing value set for linerless media offset:

```
       ! U1 getvar "media.linerless_offset"

```

**Example (for a 203 dpi linerless printer)**

This `setvar` example shows the value set to `"65"` .

```
       ! U1 setvar "media.linerless_offset" "65"

```

When the `setvar` value is set to `"65"`, the `getvar` result is `"65"` .


862


SGD Printer Commands