# SPLIT




ZBI Commands


This function allows a string to be split into sub-strings.


**Format**
```
   SPLIT(DEST$,SOURCE$,DELIMITER$)
   SPLIT(DEST$,SOURCE$,DELIMITER$,MAXCOUNT)
```

**Parameters**

    - `DEST$` = the array to populate with the sub-strings created by the split

    - `SOURCE$` = the string that will be searched for the provided delimiter

    - `DELIMITER$` = the delimiter string (may be more than one character) to search for

    - `MAXCOUNT` = the maximum number of sub-strings the string should be split into. A negative
value will return every sub-string created by the split. A value of zero will return empty strings in
the array. If not specified, the limit will be the maximum size of the array.


**Returns**
The number of sub-strings placed into the DEST$ array. If the number of sub-strings is less than the
size of DEST$, the remaining elements of the array will be set to empty strings.


548


ZBI Commands


**Example**


This is an example of how to use the SPLIT command:

```
 1 REM Example - This example show how the SPLIT and SPLITCOUNT
 1 REM commands can be
 1 REM used to merge a comma separated variable string(CSV)
 1 REM into a stored format
 AUTONUM 1,1
 SLEEP 10
 DECLARE STRING TESTDATA$(5)
 REM data format = <Format Name>,<VAR 1>,<VAR 2>,...,<VAR N>
 LET TESTDATA$(1) = "E:PRICETAG.ZPL,FRED'S OATS,
 $1.25,C:126789:325,123456789"
 LET TESTDATA$(2) = "E:PRICETAG.ZPL,FRED'S OATS,
 $2.25,C:126789:325,123456789"
 LET TESTDATA$(3) = "E:PRICETAG.ZPL,FRED'S OATS,
 $3.25,C:126789:325,123456789"
 LET TESTDATA$(4) = "E:PRICETAG.ZPL,FRED'S OATS,
 $4.25,C:123489:325,123456789"
 LET TESTDATA$(5) = "E:PRICETAG.ZPL,FRED'S OATS,
 $5.25,C:123459:325,123456789"
 LET ZPLPORT = 2
 OPEN #ZPLPORT: NAME "ZPL"
 FOR T = 1 TO 5
 LET DATA$ = TESTDATA$(T)
 GOSUB CSVPRINTER
 NEXT T
 END
 REM ******* Subroutine CSVPRINTER, expects DATA$ and ZPLPORT
 ****************
 SUB CSVPRINTER
 LET CNT = SPLITCOUNT(DATA$, ",")
 DECLARE STRING SPLITSTRING$(CNT)
 ON ERROR GOTO RECOVERY
 LET CNT = SPLIT(SPLITSTRING$,DATA$,",")
 PRINT #ZPLPORT: "^XA^XF";SPLITSTRING$(1);"^FS"
 IF CNT >= 2 THEN
 FOR I = 2 TO CNT
 PRINT #ZPLPORT: "^FN";I-1;"^FD";SPLITSTRING$(I);"^FS"
 NEXT I
 END IF
 PRINT #ZPLPORT: "^XZ"
 SUB RECOVERY
 RETURN

```

549


ZBI Commands


**Example**
This is an example of how to use the SPLIT command:

```
 1 REM Example - Shows how the SPLIT and SPLITCOUNT commands can be used
 to
 1 REM merge a comma separated variable string(CSV) into a stored
 format
 AUTONUM 1,1
 SLEEP 10
 DECLARE STRING TESTDATA$(5)
 REM data format = <Format Name>,<VAR 1>,<VAR 2>,...,<VAR N>
 LET F$="E:PRICETAG.ZPL"
 LET TESTDATA$(1) = F$&",FRED'S ROLLED OATS,
 $1.25,C:123456789:325,123456789"
 LET TESTDATA$(2) = F$&",FRED'S ROLLED OATS,
 $2.25,C:123456789:325,123456789"
 LET TESTDATA$(3) = F$&",FRED'S ROLLED OATS,
 $3.25,C:123456789:325,123456789"
 LET TESTDATA$(4) = F$&",FRED'S ROLLED OATS,
 $4.25,C:123456789:325,123456789"
 LET TESTDATA$(5) = F$&",FRED'S ROLLED OATS,
 $5.25,C:123456789:325,123456789"
 LET ZPLPORT = 2
 OPEN #ZPLPORT: NAME "ZPL"
 FOR T = 1 TO 5
 LET DATA$ = TESTDATA$(T)
 GOSUB CSVPRINTER
 NEXT T
 END
 REM ******* Subroutine CSVPRINTER, expects DATA$ and ZPLPORT
 *****************
 SUB CSVPRINTER
 LET CNT = SPLITCOUNT(DATA$, ",")
 DECLARE STRING SPLITSTRING$(CNT)
 ON ERROR GOTO RECOVERY
 LET CNT = SPLIT(SPLITSTRING$,DATA$,",")
 PRINT #ZPLPORT: "^XA^XF";SPLITSTRING$(1);"^FS"
 IF CNT >= 2 THEN
 FOR I = 2 TO CNT
 PRINT #ZPLPORT: "^FN";I-1;"^FD";SPLITSTRING$(I);"^FS"
 NEXT I
 END IF
 PRINT #ZPLPORT: "^XZ"
 SUB RECOVERY
 RETURN

```

550




ZBI Commands


**Comments**
If the delimiter is an empty string, or does not appear in the `SOURCE$` string, the first entry of the
array will be the source string and all other elements will be empty strings.

When the `SPLIT` function encounters a delimiter at the beginning or end of the source string, or
two delimiters in a row, it populates the corresponding array element with an empty string.

If `MAXCOUNT` is larger than the number of returned sub-strings (N), the last `MAXCOUNT`   - N array
elements will be empty strings. If `MAXCOUNT` is larger than the destination array or is negative, the
size of the array will be used as the `MAXCOUNT` . Therefore, the smallest value among the value
of `MAXCOUNT`, the size of the return array, or the number of sub-strings found determines the
maximum number of sub-strings that will be returned.

If `MAXCOUNT` is less than the number of delimiters in a string the last string in the array will hold
the end of the string starting from where the last delimiter was found. For example, if `SOURCE$` =
"one,two,three,four,five", `DELIMITER$` = ",", and `MAXCOUNT` = 2, the output would be two strings:
"one" and "two,three,four,five".

If a two dimensional array is provided for `DEST$`, the array will be filled linearly. For example, an
array that is 2 x 3 (for example, `DECLARE STRING MYARRAY$(2,3)` ) will be filled from (0,0), then
(0,1) up to (2,3).


551


ZBI Commands