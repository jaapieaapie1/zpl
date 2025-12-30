# ip.active_network



This command displays if the printer is actively connected to wireless, external wired, or internal wired.


**Getvar**


To return the current active network the printer is connected to:

```
       ! U1 getvar "ip.active_network"

```

The Printer Response table below details on the potential return values.


**Example**


In this example, the `getvar` will return the current active network the printer is connected to.

```
       ! U1 getvar "ip.active_network"

```

**Table 27** Printer Response

|Return Values|Details|
|---|---|
|`"internal wired"`|This is the return value when an internal wired device is detected.|
|`"wireless"`|This is the return value when a wireless device is detected.|
|`"external wired"`|This is the return value when an external wired device is detected.|
|`"unknown"`|This is the return value:<br>•<br>if the printer has not established a network connection on any of the<br>devices<br>•<br>if you don't have any of the network devices plugged in<br>•<br>if the printer is still trying to establish a connection (i.e. on wireless it is<br>going through the association process).|



1215