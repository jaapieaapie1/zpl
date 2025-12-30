# device.host_identification



This command is designed to be sent from the host to the Zebra printer to retrieve information. Upon
receipt, the printer responds with information on the model, software version, dots-per-millimeter setting,
memory size, and any detected options.

This command is equivalent to the `~HI` ZPL command.


**Getvar**


To display information about the printer:

```
       ! U1 getvar "device.host_identification"

```

**Result**
```
          "XXXXXX,V1.0.0,dpm,000KB,X"
```

**Values**

`XXXXXX` indicates the model of Zebra printer

`V1.0.0` is the version of software

`dpm` is dots/mm printheads

              - `6`

              - `8`

              - `12`

              - `24`

`000KB` is the memory size

              - `512KB` (.5 MB)

              - `1024KB` (1 MB)

              - `2048KB` (2 MB)

              - `4096KB` (4MB)

              - `8192KB` (8MB)

`x` displays options specific to printer (for example, cutter)


703


SGD Printer Commands