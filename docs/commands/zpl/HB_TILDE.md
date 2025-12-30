# ~HB




ZPL Commands


When the `~HB` command is sent to the printer, a data string is sent back to the host. The string starts with
an `<STX>` control code sequence and terminates by an `<ETX><CR><LF>` control code sequence.


**Battery Status**


**NOTE:** This command only responds to mobile printers.


**Format:** ~HB


**Parameters**


When the printer receives the command, it returns:

```
<STX>hh.hh,bb.bb,bt<ETX><CR><LF>

```

`=` ASCII start-of-text character
```
 <STX>

```

`=` current head voltage reading in integers
```
 hh.hh

```

`=` current battery voltage reading in integers
```
 bb.bb

```

`=` battery temperature in Celsius
```
 bt

```

`=` ASCII end-of-text character
```
 <ETX>

```

`=` ASCII carriage return
```
 <CR>

```

`=` ASCII line feed character
```
 <LF>

```

- This command is used for the power-supply battery of the printer and should not be confused with the
battery backed-up RAM.


- For a more precise voltage reading, you can use the power.voltage SGD command, which returns a
value to the nearest hundredths of a volt (X.XX).


218