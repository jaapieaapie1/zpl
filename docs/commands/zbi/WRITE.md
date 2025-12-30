# WRITE




ZBI Commands


This is a non-blocking output function. It will write as many bytes as the output buffer can hold.


**Format**
```
   WRITE (<CHANNEL>, <A>, <BYTES>)
```

**Parameters**

`<CHANNEL>` = reads from this port. Default = 0.

`<A$>` = the string to write out.

`<MAXBYTES>` = The number of bytes to write

**Returns**
The number of bytes written.

**Example**
This is an example of `WRITE` command:

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

500




ZBI Commands