# File System



This section shows how programs and formats can be saved and recalled. Here’s a quick list of these
commands:


**STORE**
Saves the program currently in memory as the specified file name.

**LOAD**
Transfers a program file previously stored in the printer’s memory and opens it in the ZBI Program
Memory.

**DIR**
With no filter included, prompts the printer to list all of the ZBI programs residing in all printer
memory locations.

**DELETE**
Removes a specified file from the printer’s memory.

## **Runtime Access**


The following example is a method to store runtime data in the printer memory. The file system in the
printer is limited to writing one file at a time. Since only one component of the printer can have write access
to the file system, the ZPL parser is the component with this access. For ZBI to use the ZPL parser as a
gateway into printer memory, the ZPL comment command ( `^FX` ) is used.


**Example**

```
       AUTONUM 1,1
       REM ******* TEST FOR SUBROUTINES **********************
       LET ZPLPORT = 1 OPEN #ZPLPORT: NAME "ZPL"
       LET SIZE = 5
       LET FILENAME$ = "R:TESTSYS.ZPL"
       DECLARE STRING DATAIN$(SIZE)
       LET DATAIN$(1) = "ONE"
       LET DATAIN$(2) = "TWO"
       LET DATAIN$(3) = "THREE"
       LET DATAIN$(4) = "FOUR"
       LET DATAIN$(5) = "FIVE"
       GOSUB STOREDATA
       GOSUB GETDATA
       FOR I = 1 TO SIZE
       IF DATAIN$(I) <> DATAOUT$(I) THEN
       PRINT #ZPLPORT: "^XA^FO100,100^A0N,50,50^FDERROR:";
       PRINT #ZPLPORT: DATAOUT$(I);"^XZ"
       END IF
       NEXT I
       END
       REM **** SUBROUTINE STOREDATA **************************
       REM INPUT: ZPLPORT, DATAIN$, SIZE, FILENAME$ ***********
       SUB STOREDATA
       PRINT #ZPLPORT: "^XA^DF" & FILENAME$ & "^FS"
       PRINT #ZPLPORT: "^FX"; SIZE; "^FS"
       FOR I = 1 TO SIZE
       PRINT #ZPLPORT: "^FX" & DATAIN$(I) & "^FS"

```

508


ZBI Commands

```
NEXT I
PRINT #ZPLPORT: "^XZ"
RETURN
REM **** SUBROUTINE GETDATA - **************************
REM INPUT: ZPLPORT, FILENAME$ **************************
REM ** OUTPUT: DECLARES AND FILLS DATAOUT$ AND FILLS SIZE
SUB GETDATA
PRINT #ZPLPORT: "^XA^HF" & FILENAME$ & "^XZ"
SLEEP 1
LET RESULT$ = ""
FOR J = 1 TO 25
LET A = READ(ZPLPORT,TEMP$,5000)
LET RESULT$ = RESULT$ & TEMP$
IF POS(RESULT$,"^XZ") <> 0 THEN
EXIT FOR
END IF
SLEEP 1
NEXT J
LET RESULT$(1:POS(RESULT$,"^FX")+2) = ""
LET SIZE = VAL(EXTRACT$(RESULT$,"","^"))
DECLARE STRING DATAOUT$(SIZE)
FOR I = 1 TO SIZE
LET RESULT$(1:POS(RESULT$,"^FX")+2) = ""
LET DATAOUT$(I) = EXTRACT$(RESULT$,"","^")
NEXT I
LET RESULT$ = ""
LET TEMP$ = ""
RETURN

```

509