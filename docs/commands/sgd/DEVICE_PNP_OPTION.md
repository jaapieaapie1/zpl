# device.pnp_option



This command defines the type of Plug and Play response that is sent by the printer after the printer is
started. The printer must be restarted for a new PNP string to be reported.


**Setvar**


To instruct the printer to select the desired Plug and Play response option:

```
       ! U1 setvar "device.pnp_option" "value"

```

**Values**

              - `"epl"` Eltron Programming Language

              - `"zpl"` Zebra Programming Language

**Default**
```
          "zpl"

```

**Getvar**


To retrieve the Plug and Play option setting:

```
       ! U1 getvar "device.pnp_option"

```

**Example**

This `setvar` example shows the value set to `"epl"` .

```
       ! U1 setvar "device.pnp_option" "epl"

```

When the `setvar` value is set to `"epl"`, the `getvar` result is `"epl"` .


**NOTE:** For GT800 printers only: when the printer’s Plug and Play string is set to EPL, the KDU
Plus displays ' `ONNECTION - EPL Printer (DTE)` even when set to ZPL Forms mode. This
behavior only affects the display, not the functionality.


726


SGD Printer Commands