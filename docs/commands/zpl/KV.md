# ^KV




ZPL Commands


The `^KV` command sets several parameters that affect the printers operation when `^MM` is set to K - Kiosk
mode


**Kiosk Values**


**Supported Devices:**


- KR403

**Format:** `^KVa,b,c,d,e`

|Parameters|Details|
|---|---|
|a = kiosk cut amount|**Values:**<br>0 = normal cut<br>10-60 = partial cut, value = mm of media left uncut<br>**Default:** `0`<br>This parameter is ignored if it is missing or invalid. The current value of the<br>parameter remains unchanged.|
|b = kiosk cut margin|**Values:**<br>2 - 9 = mm of distance<br>**Default:**<br>`9` = mm of distance<br>This parameter is ignored if it is missing or invalid. The current value of the<br>parameter remains unchanged.|
|c = kiosk present type|**Values:**<br>0 = Eject page when new page is printed<br>1 = Retract page when new page is printed<br>2 = Do nothing when new page is printed<br>**Default:** `0`<br>This parameter is ignored if it is missing or invalid. The current value of the<br>parameter remains unchanged.|
|d = kiosk present timeout|**Values:**<br>0–300 = If label is not taken, retract label when timeout expires. Timeout<br>is in seconds. Zero (0) indicates that there is no timeout. The label will stay<br>presented until removed manually or a new label is printed.<br>**Default:** `0`<br>This parameter is ignored if it is missing or invalid. The current value of the<br>parameter remains unchanged.|



288


ZPL Commands

|Parameters|Details|
|---|---|
|e = presenter loop length|**Values:**<br>0 = paper is fed straight through the presenter<br>3-1023 = loop length in mm.<br>**Default:** `400`<br>400= gives a loop of approximately 400mm<br>This parameter is ignored if it is missing or invalid. The current value of the<br>parameter remains unchanged. . If this is greater than loop_length_max<br>(see SGD media.present.loop_length_max) then it will be set equal to<br>loop_length_max.|



**Kiosk Printing Examples**

The following examples demonstrate the use of the `^KV`, `^CN`, `^PN` and `^CP` commands with 80 mm wide
continuous media and the printer set to Kiosk Mode ( `^MMK` ).

**Example:** In this example, the `^KV` command is set to the following:

- Cut - Full Cut


- Cut Margin - 9 mm


- Present Type - Eject page when the next page is printed


- Present Timeout - 6 seconds after printing, if the document is not taken, it will be retracted


- Presenter Loop Length - No loop

```
^XA
^MMK
^KV0,9,0,6,0
^FO50,50^A0N,50,50^FDZebra Technologies^FS
^CN1
^PN0
^XZ

```

**NOTE:** The ^CN1 command (Cut Now) is included to ensure that a full cut is done. The ^PN0
(Present Now) command is included to ensure that the media is ejected when the user pulls on
the leading edge of the media. In this example, if the user does not pull on the leading edge of
the second document, it will be retracted.


**Example:** This example contains only one change from the Example 1 - the Presenter Loop Length is now
100mm, and two documents will be printed instead of one.

```
^XA
^MMK
^KV0,9,2,6,100
^FO50,50^A0N,50,50^FDZebra Technologies^FS
^CN1^PN0
^PQ2
^XZ

```

**Example:** In this example, two documents will be printed, each one will be ejected from the printer.


289


ZPL Commands

```
^XA
^MMK
^KV0,9,2,6,100
^FO50,50^A0N,50,50^FDZebra Technologies^FS
^CN1^CP0
^PQ2
^XZ

```

**Example:** In this example, two documents, with partial cuts, will be printed, and a third document, with a full
cut, will be printed.

```
^XA
^MMK
^KV50,9,0,0,0
^FO50,50^A0N,50,50^FDPartial Cut^FS
^CN0^PN0
^PQ2
^XZ
^XA
^MMK
^KV0,9,2,6,0
^FO50,50^A0N,50,50^FDFull Cut^FS
^CN1^CP0
^XZ

```

**Example:** In this example, four documents will be printed – three with a partial cut and the fourth with
a full cut. Additionally, the document length is set to 406 dots and the Media Tracking mode is set to
"Continuous Media, Variable Length". The third document contains fields that are positioned outside of
the 406 dot length – however, because the printer is set to “Continuous Media, Variable Length" Media
Tracking mode, the printer will automatically adjust the document length to compensate.

```
^XA
^MMK
^LL406
^KV20,9,0,0,0
^FO50,50^A0N,50,50^FDPartial Cut^FS
^CN0^PN0
^PQ2
^XZ
^XA
^MMK
^MNV
^KV20,9,0,0,0
^FO50,50^A0N,50,50^FDPartial Cut^FS
^FO50,150^A0N,50,50^FDPrinting Line 1^FS
^FO50,250^A0N,50,50^FDPrinting Line 2^FS
^FO50,350^A0N,50,50^FDPrinting Line 3^FS
^FO50,450^A0N,50,50^FDPrinting Line 4^FS
^FO50,550^A0N,50,50^FDPrinting Line 5^FS
^FO50,650^A0N,50,50^FDPrinting Line 6^FS
^FO50,750^A0N,50,50^FDPrinting Line 7^FS
^FO50,850^A0N,50,50^FDPrinting Line 8^FS

```

290




ZPL Commands

```
^FO50,950^A0N,50,50^FDPrinting Line 9^FS
^FO50,1050^A0N,50,50^FDPrinting Line 10^FS
^FO50,1150^A0N,50,50^FDPrinting Line 11^FS
^FO50,1250^A0N,50,50^FDPrinting Line 12^FS
^FO50,1350^A0N,50,50^FDPrinting Line 13^FS
^FO50,1450^A0N,50,50^FDPrinting Line 14^FS
^FO50,1550^A0N,50,50^FDPrinting Line 15^FS
^CN0^PN0
^XZ
^XA
^MMK
^KV0,9,0,0,0
^FO50,50^A0N,50,50^FDFull Cut^FS
^CN0^PN1^CP0
^PQ1
^XZ

```

291