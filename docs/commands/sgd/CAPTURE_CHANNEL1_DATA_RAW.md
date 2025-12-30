# capture.channel1.data.raw



This command retrieves the user data captured off of the port specified in `capture.channel1.port` .

Any binary zeros in the capture.data stream will be replaced with the escaped representation of NULL
("\000"). The delimiter data is not stored as part of the captured data.

This will be shown in the `HZA` output within capture data section.


**Getvar**

To retrieve the user data captured off of the port specified in `capture.channel1.port` :

```
       ! U1 getvar "capture.channel1.data.raw"

```

644


SGD Printer Commands