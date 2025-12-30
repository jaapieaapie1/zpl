# netmanage.avalanche.encryption_type



This parameter sets and gets the Network Management Encryption type to be used.


**Setvar**


To set the Network Management Encryption type to be used:

```
       ! U1 setvar "netmanage.avalanche.encryption_type" "value"

```

**Values**

              - `"0"` None

              - `"1"` Limburger

              - `"2"` AES 128S

**Default**
```
          "0"

```

**Getvar**


To retrieve the currently set Network Management Encryption type:

```
       ! U1 getvar "netmanage.avalanche.encryption_type"

```

**Example**


This example sets the value to Limburger (1) encryption type.

```
       ! U1 getvar "netmanage.avalanche.encryption_type" "1"

```

902


SGD Printer Commands