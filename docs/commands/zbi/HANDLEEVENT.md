# HANDLEEVENT



Once events have been registered, this function is used to see what events have occurred.


**Format**
```
          HANDLEEVENT()
```

**Parameters**
N/A

**Returns**
The ID of the event that occurred. One event at a time will be returned through this function. The
order of the events are based on priority. The priority is based on the ID number of the event, with
the exception of the timer events, which have the highest priority.


528


ZBI Commands


**Example**
Here are examples of how to use the `HANDLEEVENT` command:

```
 1 REM This example shows how to override the feed key functionality
 1 REM using the event system. Why waste a label when you could put
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
 REM ******** SUBROUTINE PRINTINFO ***
 REM *** expects ZPLPORT *************
 SUB PRINTINFO
 PRINT #ZPLPORT: "^XA"
 PRINT #ZPLPORT: "^FO30,30^A0N,50,50";
 PRINT #ZPLPORT: "^FDZebra Technologies^FS"
 PRINT #ZPLPORT: "^FO30,85^A0N,35,35";
 PRINT #ZPLPORT: "^FDwww.zebra.com^FS"
 PRINT #ZPLPORT: "^FO30,125^A0N,35,35";
 PRINT #ZPLPORT: "^FDsupport.zebra.com^FS"
 PRINT #ZPLPORT: "^FO30,165^A0N,35,35";
 PRINT #ZPLPORT: "^FDFW Version: ";
 PRINT #ZPLPORT: GETVAR$("appl.name") & "^FS"
 PRINT #ZPLPORT: "^FO30,205^A0N,35,35";
 PRINT #ZPLPORT: "^FDPrinter Unique ID:";
 PRINT #ZPLPORT: GETVAR$("device.unique_id") & "^FS"
 PRINT #ZPLPORT: "^FO30,245^A0N,35,35";
 PRINT #ZPLPORT: "^FDActive Network: ";
 PRINT #ZPLPORT: GETVAR$("ip.active_network") & "^FS"
 PRINT #ZPLPORT: "^FO30,285^A0N,35,35";
 PRINT #ZPLPORT: "^FDZBI Memory Usage: ";
 PRINT #ZPLPORT: GETVAR$("zbi.start_info.memory_alloc") & "^FS"
 PRINT #ZPLPORT: "^FO30,325^A0N,35,35";
 PRINT #ZPLPORT: "^FDOdometer: ";
 PRINT #ZPLPORT: GETVAR$("odometer.total_print_length") & "^FS"
 PRINT #ZPLPORT: "^XZ"

```

**Comments**
None


529


ZBI Commands