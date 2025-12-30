# zpl.system_error



This command returns the system error flags.

This command is equivalent to the error recording of `~HQES` .


**Getvar**


To return the state of the system error flags:

```
       ! U1 getvar "zpl.system_error"

```

**Result**

```
          "0,0,00000000,00000000"

```

(flag, error flag, Group 2, Group 1)


**Example**


This example shows how to request the printer’s status.

To request the printer’s status, type `! U1 getvar "zpl.system_error"`

The printer responds with data similar to this:

```
       "1,1,00000000,00000004"

```

In this example, the Printer Status resolves to these conditions:


          - The printer is in Pause (value = 1)


          - The Error Flag is 0 if there are no errors (i.e. Group 2 and Group 1 are all 0s), and 1 if there are any
errors.


          - The cover/printhead is open (value = 4).


**Table 24** Error Flags (~HQES)





|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles16-9**|**Nibble8**|** Nibble7**|** Nibble6**|**  Nibble5**|**  Nibble4**|**   Nibble3**|**   Nibble2**|**    Nibble1**|
|No Error|0|00000000|0|0|0|0|0|0|0|0|
|Error Present|1|00000000|X|X|X|X|X|X|X|X|
|Printhead<br>Thermistor Open|1|00000000|X|X|X|X|X|2|X|X|
|Invalid Firmware<br>Config.|1|00000000|X|X|X|X|X|1|X|X|
|Printhead<br>Detection Error|1|00000000|X|X|X|X|X|X|8|X|
|Bad Printhead<br>Element|1|00000000|X|X|X|X|X|X|4|X|


1078


SGD Printer Commands


**Table 24** Error Flags (~HQES) (Continued)



















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
|Black Mark<br>Calibrate Error1||00000000|X|X|X|4|X|X|X|X|
|Retract Function<br>timed out1|1|00000000|X|X|X|2|X|X|X|X|
|Paused1|1|00000000|X|X|X|1|X|X|X|X|


1. This error flag is supported only on KR403 printers.


1079


SGD Printer Commands