# CLOSE




ZBI Commands


This command is implemented to close specific ports that are in use. If a port is open on a channel and the
`CLOSE` command is entered, the port closes and returns to communicating with the ZPL buffer.

**Format**
```
   CLOSE #<A>
   CLOSE ALL
```

**Parameters**
`<A>` = Numeric value of port to close

**Values**


0 through 9

`All` = closes all open ports and network connections


**NOTE:** `CLOSE ALL` will close the console.


**Example**
This example shows the closing of channel 1:This example shows the closing of channel 1:

```
10 CLOSE #1

```

**Comments**
This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.


487


ZBI Commands