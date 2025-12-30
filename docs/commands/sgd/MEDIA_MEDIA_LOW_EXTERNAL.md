# media.media_low.external



This printer setting gets the status of the external `media.media_low` warning.


**Getvar**


To instruct the printer to respond with the currently set media print mode:

```
       ! U1 getvar "media.media_low.external"

```

**Values**

              - `"0"` indicates paper present at sensor position

              - `"1"` indicates no paper present


**NOTE:** The status of the sensor is sampled every time the printout is cut. If three
succeeding samples show "no paper", the status reply changes to 1. This is to prevent a
false alarm if the side of the paper roll is not clean. If the current status of the sensor is
required, use `~HQES` and extract the paper near-end sensor bit.


863


SGD Printer Commands