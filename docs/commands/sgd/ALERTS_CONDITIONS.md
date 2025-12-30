# alerts.conditions



This command lists the available conditions that can be specified in the first parameter of the `alerts.add`
SGD. See `alerts.add` for information on the various parameters.


**Getvar**


To retrieve the list of available alert conditions for the printer:

```
       ! U1 getvar "alerts.conditions"

```

**Values**


PAPER OUT, RIBBON OUT, HEAD TOO HOT, HEAD COLD, HEAD OPEN, SUPPLY TOO HOT,
RIBBON IN, REWIND, CUTTER JAM, MED, PRINTER PAUSED, PQ JOB COMPLETED, LABEL READY,
HEAD ELEMENT BAD, BASIC RUNTIME, BASIC FORCED, POWER ON, CLEAN PRINTHEAD, MEDIA
LOW, RIBBON LOW, REPLACE HEAD, BATTERY LOW, RFID ERROR, ALL MESSAGES, COLD START,
SGD SET


**Default**
```
          ""

```

611


SGD Printer Commands