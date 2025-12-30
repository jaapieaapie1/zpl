# power.dtr_power_off



This command refers to the remote printer power control, and is used for power management. When Data
Terminal Ready (DTR) is enabled the printer can be powered on and off via the Data Set Ready (DSR)
signal. When DTR power off is enabled, a low to high transition will cause the printer to turn ON and a high
to low transition will cause the printer to turn OFF.


**NOTE:** The inactivity time-out is disabled while DSR is active.


**Setvar**


To turn DTR power on or off:

```
       ! U1 setvar "power.dtr_power_off" "value"

```

**Values**
```
          "off"
          "on"
```

**Default**
```
          "on"

```

**Getvar**


To retrieve the current DTR power-off setting:

```
       ! U1 getvar "power.dtr_power_off"

```

**Example**

```
       ! U1 setvar "power.dtr_power_off" "off"

```

946


SGD Printer Commands