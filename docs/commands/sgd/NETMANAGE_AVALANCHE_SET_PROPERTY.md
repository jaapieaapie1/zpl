# netmanage.avalanche.set_property



This parameter sets Network Management Device Side Property (custom).


**Setvar**


To set the Network Management Device Side Property (custom):

```
       ! U1 setvar "netmanage.avalanche.set_property" "value"

```

**Values**

A string in the format of `"AAAA=XXXXXXXX"`


**Getvar**


To retrieve the current Network Management Device Side Property value:

```
       ! U1 getvar "netmanage.avalanche.set_property"

```

**Example**


This example will be viewed as a property under the general properety tree in avalanche console.

```
       ! U1 setvar "netmanage.avalanche.set_property" "ZebraLocation=VH"

```

This example will be viewed as a property under the Zebra tree in avalanche console.

```
       ! U1 setvar "netmanage.avalanche.set_property" "Zebra.Location=VH"

```

906


SGD Printer Commands