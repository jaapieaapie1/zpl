# SETERR




ZBI Commands


This command sends a message to the printer to set the error flag. A logical interpreter flag is triggered in
the printer. This error is referenced as a BASIC Forced Error.


**Format**
```
   SETERR
```

**Parameters**
N/A

**Example**
An example of the `SETERR` and `CLRERR` commands.

```
   AUTONUM
   1,1
   OPEN #1:NAME "ZPL"
   PRINT #1: "^XA^SXO,A,Y,Y^XZ"
   CLOSE #1
   FOR I=1 TO 10
   SLEEP 5
   IF MOD(I,2)=1 THEN
   SETERR
   ELSE
   CLRERR
   ENDIF
   NEXT I

```

**Comments**
This is a program command and must be preceded by a line number.


534