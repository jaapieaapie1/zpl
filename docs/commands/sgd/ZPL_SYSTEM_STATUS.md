# zpl.system_status



This command returns the errors and warnings of the system.

This command is equivalent to all data reported by the `~HQES` ZPL command.


**Getvar**


To return the system error and warning flags:

```
       ! U1 getvar "zpl.system_status"

```

**Result**
```
          "0","0","00000000","00000000","0","00000000","00000000"
```

(flag, error flag, group 2, group 1, warning flag, group 2, group 1)


**Example**


This example shows how to request the printer’s status.

To request the printer’s status, type `! U1 getvar "zpl.system_status"`

The printer responds with data similar to this:

```
       "1,1,00000000,00000004,0,00000000,00000000"

```

In this example, the Printer Status resolves to these conditions:


          - The printer is in Pause (value = 1)


          - The Error Flag is 0 if there are no errors (i.e. Group 2 and Group 1 are all 0s), and 1 if there are any errors
(non-zero).


          - The cover/printhead is open (value = 4).


          - The Warning Flag is 0 if there are no warnings (i.e. Group 2 and Group 1 are all 0s), and 1 if there are any
errors (non-zero).


**Table 25** Error Flags (~HQES)





|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles16-9**|**Nibble8**|** Nibble7**|** Nibble6**|**  Nibble5**|**  Nibble4**|**   Nibble3**|**   Nibble2**|**    Nibble1**|
|No Error|0|00000000|0|0|0|0|0|0|0|0|
|Error Present|1|00000000|X|X|X|X|X|X|X|X|
|Printhead<br>Thermistor Open|1|00000000|X|X|X|X|X|2|X|X|
|Invalid Firmware<br>Config.|1|00000000|X|X|X|X|X|1|X|X|
|Printhead<br>Detection Error|1|00000000|X|X|X|X|X|X|8|X|
|Bad Printhead<br>Element|1|00000000|X|X|X|X|X|X|4|X|


1080




SGD Printer Commands


**Table 25** Error Flags (~HQES) (Continued)



















|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles16-9**|**Nibble8**|** Nibble7**|** Nibble6**|**  Nibble5**|**  Nibble4**|**   Nibble3**|**   Nibble2**|**    Nibble1**|
|Motor Over<br>Temperature|1|00000000|X|X|X|X|X|X|2|X|
|Printhead Over<br>Temperature|1|00000000|X|X|X|X|X|X|1|X|
|Cutter Fault|1|00000000|X|X|X|X|X|X|X|8|
|Head Open|1|00000000|X|X|X|X|X|X|X|4|
|Ribbon Out|1|00000000|X|X|X|X|X|X|X|2|
|Media Out|1|00000000|X|X|X|X|X|X|X|1|
|Clear Paper Path<br>Failed1|1|00000000|X|X|X|X|8|X|X|X|
|Paper Feed Error1|1|00000000|X|X|X|X|4|X|X|X|
|Presenter Not<br>Running1|1|00000000|X|X|X|X|2|X|X|X|
|Paper Jam during<br>Retract1|1|00000000|X|X|X|X|1|X|X|X|
|Black Mark not<br>Found1|1|00000000|X|X|X|8|X|X|X|X|
|Black Mark<br>Calibrate Error1|1|00000000|X|X|X|4|X|X|X|X|
|Retract Function<br>timed out1|1|00000000|X|X|X|2|X|X|X|X|
|Paused1|1|00000000|X|X|X|1|X|X|X|X|


1. This error flag is only supported on KR403 printers.


**Table 26** Warning Flags (~HQES)











|Warning Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Warning Flags**|**Flag**|**Nibbles16-9**|**Nibble8**|** Nibble7**|** Nibble6**|**  Nibble5**|**  Nibble4**|**   Nibble3**|**   Nibble2**|**    Nibble1**|
|No Warning|0|00000000|0|0|0|0|0|0|0|0|
|Warning Present|1|00000000|X|X|X|X|X|X|X|X|
|Paper-near-end<br>Sensor1|1|00000000|X|X|X|X|X|X|X|8|
|Replace Printhead|1|00000000|X|X|X|X|X|X|X|4|
|Clean Printhead|1|00000000|X|X|X|X|X|X|X|2|
|Need to Calibrate<br>Media|1|00000000|X|X|X|X|X|X|X|1|


1081


SGD Printer Commands


**Table 26** Warning Flags (~HQES) (Continued)







|Warning Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Warning Flags**|**Flag**|**Nibbles16-9**|**Nibble8**|** Nibble7**|** Nibble6**|**  Nibble5**|**  Nibble4**|**   Nibble3**|**   Nibble2**|**    Nibble1**|
|Sensor 1 (Paper<br>before head)1|1|00000000|X|X|X|X|X|X|1|X|
|Sensor 2 (Black<br>mark)1|1|00000000|X|X|X|X|X|X|2|X|
|Sensor 3 (Paper<br>after head)1|1|00000000|X|X|X|X|X|X|4|X|
|Sensor 4 (loop<br>ready)1|1|00000000|X|X|X|X|X|X|8|X|
|Sensor 5<br>(presenter)1|1|00000000|X|X|X|X|X|1|X|X|
|Sensor 6 (retract<br>ready)1|1|00000000|X|X|X|X|X|2|X|X|
|Sensor 7 (in<br>retract)1|1|00000000|X|X|X|X|X|4|X|X|
|Sensor 8 (at bin)1|1|00000000|X|X|X|X|X|8|X|X|


1. This warning flag is only supported on KR403 printers.


1082


SGD Printer Commands

## **zpl.zpl_mode**


This command sets the ZPL mode to ZPL II or ZPL.


**Setvar**


To set the printer ZPL mode:

```
       ! U1 setvar "zpl.zpl_mode" "value"

```

**Values**

              - `"zpl"`

              - `"zpl II"`

**Default**
```
          "zpl II"

```

**Getvar**


To return the current ZPL mode setting:

```
       ! U1 getvar "zpl.zpl_mode"

```

**Example**

This `setvar` example sets the ZPL mode to ZPL.

```
       ! U1 setvar "zpl.system_status" "zpl"

```

1083


SGD Printer Commands

## **zpl.zpl_override**


Enable this menu item to prevent the following ZPL commands from changing the printer’s current settings:

          - `^MM` (print mode)

          - `^MT` (Direct Thermal or Thermal Transfer print method)

          - `^MN` (media type - non-continuous or continuous)

When this menu item is disabled, these commands override the printer’s settings.


**Setvar**


To set the override status to the specified value:

```
       ! U1 setvar "zpl.zpl_override" "value"

```

**Values**

              - `"disabled"` allows override

              - `"enabled"` prevents ZPL commands from overriding printer settings.

**Default**
```
          "disabled"

```

**Example**

This example enables `zpl.zpl_override`, which prevents `^MM`, `^MT`, and `^MN` from making changes to
the current printer settings.

```
       ! U1 setvar "zpl.zpl_override" "enabled"

```

1084


SGD Printer Commands

## **zpl.relative_darkness**


Changes the relative darkness for ZPL labels. This command is similar to the ZPL `^MD` command.


**IMPORTANT:** This value is saved permanently on Desktop printers, but it is not saved
permanently on Industrial or Mobile.


**Setvar**


To set the relative darkness for ZPL labels:

```
       ! U1 setvar "zpl.relative_darkness" "value"

```

**Values**

`""` to `"300"`

**Default**
```
          "0"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "relative_darkness"

```

**Example**


These examples show setting the printer to different darkness levels:

          - If the current value shown on the configuration label is `"16"`, using a `zpl.relative_darkness`
command of `"-90"` decreases the value to `"7.0"` .

          - If the current value shown on the configuration label is `"1"`, using a `zpl.relative_darkness`
command of `"153"` increases the value to `"16.3"` .

          - If the current value shown on the configuration label is `"25"`, using a `zpl.relative_darkness`
command of `"105"` increases the value to `"30.0"`, which is the maximum value allowed.

Each `zpl.relative_darkness` command is treated separately in relation to the current value as printed
on the configuration label.


1085