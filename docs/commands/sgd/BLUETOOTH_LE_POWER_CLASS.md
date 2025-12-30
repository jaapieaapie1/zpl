# bluetooth.le.power_class



This command sets the maximum transmit power level of the Bluetooth low energy radio.


**Setvar**


To set the power class value of the bluetooth low energy printers:

```
       ! U1 setvar "bluetooth.le.power_class" "value"

```

**Values**


The values varies printer to printer as given below.

              - `"1"` ZQ610, ZQ620, ZQ630, ZR658, ZR668, ZT510, ZT610, ZT620

              - `"2"` ZQ610, ZQ620, ZQ630, ZR658, ZR668, ZT510, ZT610, ZT620

              - `"3"` ZT510, ZT610, ZT620

**Default Value**


The default values varies printer to printer as given below.

              - `"2"` ZQ610, ZQ620, ZQ630, ZR658, ZR668

              - `"3"` ZT510, ZT610, ZT620


**Getvar**


To return the current setting:

```
       ! U1 getvar "bluetooth.le.power_class"

```

1103


SGD Network Commands