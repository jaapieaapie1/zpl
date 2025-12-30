# bluetooth.power_class



This command sets the maximum transmit power level of the Bluetooth radio. If the Bluetooth radio is
configured for `"1"`, then the maximum power level of the radio is 20 dBm. If the radio is configured for
`"2"`, then the maximum power level of the radio is 4 dBm.


**Setvar**


To set the bluetooth power class value for the printer:

```
       ! U1 setvar "bluetooth.power_class" "value"

```

**Values**

`"1"` iMZ220, iMZ320, ZD500, ZQ610, ZQ620, ZQ630, ZR658, ZR668, ZT210, ZT220, ZT410,
ZT411, ZT420, ZT421, ZT510, ZT610, ZT620

`"2"` iMZ220, iMZ320, ZD500, ZQ610, ZQ620, ZQ630, ZR658, ZR668, ZT210, ZT220, ZT410,
ZT411, ZT420, ZT421, ZT510, ZT610, ZT620

`"3"` ZT510, ZT610, ZT620


**Default**

`"2"` ZQ610, ZQ620, ZQ630, ZR658, ZR668, iMZ220 Non-Sdio radio, iMZ320 Non-Sdio
radio, ZT410 Non-Sdio Radio, ZT411 Non-Sdio Radio, ZT420 Non-Sdio Radio, ZT510
8887 Sdio Radio, ZT620 Non-Sdio Radio

`"1"` ZT620 8787 Sdio Radio, ZT610 8787 Sdio Radio, ZT421 8787 Sdio Radio, ZT420 8787
Sdio Radio, ZT410 Sdio Radio, ZD500 8787 Sdio Radio


**Getvar**


To return the current setting:

```
       ! U1 getvar "bluetooth.power_class"

```

1101


SGD Network Commands