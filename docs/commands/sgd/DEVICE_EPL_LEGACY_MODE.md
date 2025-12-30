# device.epl_legacy_mode



This command places the printer in a 2824/2844 compatibility mode for vertical registration.


**Setvar**


To instruct the printer to change the epl_legacy_mode setting:

```
       ! U1 setvar "device.epl_legacy_mode" "value"

```

**Values**

              - `"off"` epl_legacy_mode not active

              - `"registration"` EPL legacy registration mode on

              - `"print orientation"` EPL legacy print orientation mode on

              - `"all"` all EPL legacy modes on

**Default**
```
          "off"

```

**Getvar**


To return the current setting value for the device.epl_legacy_mode setting:

```
       ! U1 getvar "device.epl_legacy_mode"

```

**Example**

This `setvar` example shows the value set to `"registration"` .

```
       ! U1 setvar "device.epl_legacy_mode" "registration"

```

This `setvar` example shows the value set to `"print_orientation"` .

```
       ! U1 setvar "device.epl_legacy_mode" "print_orientation"

```

This getvar example shows the response when the value was set to `"registration"` and
`"print_orientation"` .

```
       ! U1 getvar "device.epl_legacy_mode"
       "registration, print_orientation"

```

This getvar example shows the response when value was set to `"all"` .

```
       ! U1 getvar "device.epl_legacy_mode"
       "all"

```

686


SGD Printer Commands


**Example**

This `setvar` example shows the value set to `"registration"` .

```
! U1 setvar "device.epl_legacy_mode" "registration"

```

This `setvar` example shows the value set to `"print_orientation"` .

```
! U1 setvar "device.epl_legacy_mode" "print_orientation"

```

This `getvar` example shows the response when the value was set to `"registration"` and
`"print_orientation"` .

```
! U1 getvar "device.epl_legacy_mode"
"registration, print_orientation"

```

This `getvar` example shows the response when value was set to `"all"` .

```
! U1 getvar "device.epl_legacy_mode"
"all"

```

**NOTE:**

- This setting is not defaulted as part of a factory default ( `^JUF` or `^default)` . The setting is
persistent across a power cycle or reset ( `~JR` or `device.reset` ).

- When setting the “registration” mode, the `"print_orienation"` mode is not changed.
Likewise, when setting the `"print_orienation"` mode, the `"registration"` mode is
not changed. Using `"off"` or `"all"` changes all modes.


**Print Orientation Mode**


**NOTE:** When the printer is powered on, the print orientation defaults to `^PON` (EPL ZB mode). The
print orientation setting is not saved across power cycles. This is different than TLP2844, LP2844,
TLP2824, LP2824, and TLP3842 printers. Those printers have a default of ZB (ZPL `^PON` mode)
and the print orientation setting is saved across power cycles. To make the printer have the print
orientation behavior of the TLP2844, LP2844, TLP2824, LP2824, and TLP3842 printers, set the
epl_legacy_mode to `"print_orienation"` .


**Registration Mode**


**NOTE:** When printing labels using EPL commands, printing starts 1mm from the top edge of the
label (from the gap). This is known as the “no print zone”. When printing in ZT mode, the “no print
zone” starts at the gap on the leading edge of the label. When printing in ZB mode, the “no print
zone” starts from the gap on the trailing edge of the label. In the TLP2844, LP2844, TLP2824,
LP2824, and TLP3842 printers, the distance from gap to start of print (the “no print zone”) is not
always 1mm.


The table below shows the nominal distance.


687


SGD Printer Commands

|Distance from Edge of Label to First Print Line (No Print Zone)|Col2|Col3|Col4|
|---|---|---|---|
|**Legacy Printer Model**|**New Printer Model**|**ZT Mode**|**ZB Mode**|
|LP2844|GX420, GK420 (direct thermal)|1.9 mm|0.0 mm|
|TLP2844|GX420, GK420 (thermal transfer)|0.4 mm|1.6 mm|
|TLP3842|GX430 (thermal transfer)|0.0 mm|1.2 mm|
|LP2824|LP 2824 Plus (direct thermal)|1.5 mm|0.4 mm|
|TLP2824|TLP 2824 Plus (thermal transfer)|0.1 mm|1.8 mm|
|**NOTE:** Setting epl_legacy_mode to`"registration"` selects the distance shown in the<br>table. Setting epl_legacy_mode to`"off"` selects a no print zone distance of 1mm.|**NOTE:** Setting epl_legacy_mode to`"registration"` selects the distance shown in the<br>table. Setting epl_legacy_mode to`"off"` selects a no print zone distance of 1mm.|**NOTE:** Setting epl_legacy_mode to`"registration"` selects the distance shown in the<br>table. Setting epl_legacy_mode to`"off"` selects a no print zone distance of 1mm.|**NOTE:** Setting epl_legacy_mode to`"registration"` selects the distance shown in the<br>table. Setting epl_legacy_mode to`"off"` selects a no print zone distance of 1mm.|



688


SGD Printer Commands