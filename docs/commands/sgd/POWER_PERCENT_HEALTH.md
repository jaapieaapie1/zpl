# power.percent_health



This command returns the percent health that is read from the fuel gauge for printers that support a Power
Precision Plus battery. The battery health is expressed as a percentage of design capacity.


**Getvar**


To get the Power Precision battery health percentage:

```
       ! U1 getvar "power.percent_health"

```

**Result**

`"0"` to `"100"`


**Example**

In the example below, the `getvar` returns the Power Precision battery health percentage.

```
       ! U1 getvar "power.percent_health" "90"

```

956


SGD Printer Commands