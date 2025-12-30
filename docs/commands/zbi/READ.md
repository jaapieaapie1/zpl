# READ




ZBI Commands


This is a non-blocking input function. It will read in all of the bytes available on the specified port.


**Format**
```
   READ (<CHANNEL>, <A>, <MAXBYTES>)
```

**Parameters**
`<CHANNEL>` = reads from this port. Default = 0.

`<A$>` = the string where the data will be placed

`<MAXBYTES>` = the maximum number of bytes to read

**Returns**
The number of bytes read.

**Example**
This is an example of the `READ` command:

```
   1 CLOSE ALL
   2 LET INPORT = CLIENTSOCKET("TCP","192.168.0.1",9100)
   3 ON ERROR GOTO RECOVERY
   4 LET WATERMARK = 5000
   5 DO WHILE 1
   6 IF LEN(DATA$) < WATERMARK THEN
   7 LET BYTESREAD = READ(INPORT,DATA$,500)
   8 ON ERROR GOTO RECOVERY
   9 END IF
   10 IF (LEN(DATA$) > 0) THEN
   11 LET BYTES_WRITTEN = WRITE(INPORT,DATA$,LEN(DATA$))
   12 ON ERROR GOTO RECOVERY
   13 LET DATA$(1,BYTES_WRITTEN) = ""
   14 END IF
   15 IF BYTESREAD = 0 AND BYTESWRITTEN = 0 THEN
   16 SLEEP 1 ! DON'T BOMBARD IF IDLE
   17 END IF
   18 LOOP
   19 SUB RECOVERY
   20 CLOSE #INPORT

```

499