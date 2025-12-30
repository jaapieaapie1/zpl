# REGISTEREVENT



This function will set up the `HANDLEEVENT` function to receive notification when the specified event has
occurred. Events can be registered for one time or until the program is exited.


**NOTE:** If an event occurs twice or more before the `HANDLEEVENT` function is called, only one
event will be received.

**Format**

              - `REGISTEREVENT(X)`

              - `REGISTEREVENT(X,Y)`

              - `REGISTEREVENT(X,Y,Z)`

**Parameters**

              - `(X)` = This is the ID of the event being registered for.

              - `(Y)` = If Y=1: the event happens once; If Y=0: the event stays registered for the duration of the
program, or until it is unregistered.

              - `(Z)` = For System Events: if Z=0, the event will still be handled by the printer. If Z=1, then only
ZBI will receive the event.


For Timer Events: this is the timer interval in mSec. If the interval is less than 0 or greater than
1,000,000,000, it is set to 1000.


**Returns**
The ID of the successfully registered event. If an event was not successfully registered, a is
-1
returned.


525


ZBI Commands


**Example**
Here is an example of how to use the REGISTEREVENT command:

```
 1 REM This example shows how to override the functionality of the feed
 1 REM key
 1 REM using the event system. After all why waste a label when you
 1 REM could put
 1 REM valuable information there
 AUTONUM 1,1
 CLOSE ALL
 LET ZPLPORT = 1
 OPEN #ZPLPORT: NAME "ZPL"
 LET FEEDKEY = 3
 LET TMP = REGISTEREVENT(FEEDKEY, 0, 1)
 DO WHILE 1 = 1
 LET EVT = HANDLEEVENT()
 IF EVT = FEEDKEY THEN
 GOSUB PRINTINFO
 END IF
 SLEEP 1
 LOOP
 REM **** SUBROUTINE PRINTINFO *** expects ZPLPORT *******
 SUB PRINTINFO
 PRINT #ZPLPORT: "^XA"
 PRINT #ZPLPORT: "^FO30,30^A0N,50,50^FDZebra Technologies^FS"
 PRINT #ZPLPORT: "^FO30,85^A0N,35,35^FDwww.zebra.com^FS"
 PRINT #ZPLPORT: "^FO30,125^A0N,35,35^FDsupport.zebra.com^FS"
 PRINT #ZPLPORT: "^FO30,165^A0N,35,35^FDFW Version: "
 PRINT #ZPLPORT: GETVAR$("appl.name") & "^FS"
 PRINT #ZPLPORT: "^FO30,205^A0N,35,35^FDPrinter Unique ID:"
 PRINT #ZPLPORT: GETVAR$("device.unique_id") & "^FS"
 PRINT #ZPLPORT: "^FO30,245^A0N,35,35^FDActive Network: "
 PRINT #ZPLPORT: GETVAR$("ip.active_network") & "^FS"
 PRINT #ZPLPORT: "^FO30,285^A0N,35,35^FDZBI Memory Usage: "
 PRINT #ZPLPORT: GETVAR$("zbi.start_info.memory_alloc") & "^FS"
 PRINT #ZPLPORT: "^FO30,325^A0N,35,35^FDOdometer: "
 PRINT #ZPLPORT: GETVAR$("odometer.total_print_length") & "^FS"
 PRINT #ZPLPORT: "^XZ"

```

**Comments**
None


526


ZBI Commands