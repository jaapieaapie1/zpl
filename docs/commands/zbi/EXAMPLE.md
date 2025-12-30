# Example Programs



The next section provides example programs of common tasks using ZBI commands.


[These programs are also available for download at: www.zebra.com/zbi](http://www.zebra.com/zbi)

## **Array Program**


This program prompts a user to enter first a name; when it is entered, it is added to an array of all names
entered. The user is them prompted to enter an address, which is then added to an array of all addresses
entered. After the user enters a total or five names and addresses, the program uses the arrays to print the
entered data on five labels.


**Example**


This is an example of Array

```
       1 rem ********************************************************
       1 rem Zebra Technologies ZBI Sample Program
       1 rem
       1 rem Professional programming services are available. Please contact
       1 rem ZBI-Experts@zebra.com for more information.
       1 rem
       1 rem This is an example of using arrays to store and use data within 1 rem
       ZBI.
       1 rem ********************************************************
       1 rem close all ports except for the console
       1 rem*********************************************************
       10 for i = 1 to 9 step 1
       20  close #i
       30  next i
       1 rem ********************************************************
       1 rem open a port to the print engine
       1 rem ******************************************************
       40 open #1: name "ZPL"
       1 rem ********************************************************
       1 rem create string arrays five elements in size to hold names and
       1 rem addresses
       1 rem ********************************************************
       50 declare string name$(5)
       60 declare string address$(5)
       1 rem ********************************************************
       1 rem infinite loop to put name and address data from console into
       1 rem arrays
       1 rem ********************************************************
       70 do
       80 for i = 1 to 5 step 1
       90  print "PLEASE ENTER THE NAME"
       1 rem ********************************************************
       1 rem get data from console; input command looks for CRLF
       1 rem ********************************************************
       100  input name$(i)
       1 rem ********************************************************

```

586


ZBI Commands

```
     1 rem if the user inputs end or END, the program will end
     1 rem ********************************************************
     110  if name$(i) = "END" or name$(i) = "end" then
     120    end
     130  end if
     140  print "PLEASE ENTER THE ADDRESS"
     150  input address$(i)
     160  if address$(i) = "END" or address$(i) = "end" then
     170    end
     180  end if
     190 next i
     200 for index = 1 to 5 step 1 ! For loop To Print data no label
     1 rem ********************************************************
     1 rem semicolon at the end prints with no CRLF
     1 rem ********************************************************
     210  print #1: "^XA^FO30,30^A0N,30,30^FD"&NAME$(INDEX)&"^FS";
     1 rem ********************************************************
     1 rem ampersand used to concatenate data into strings
     1 rem ********************************************************
     220  print #1: "^FO30,70^A0N,30,30^FD"&ADDRESS$(INDEX)&"^FS^XZ"
     230 next index
     240 loop ! loops back To Line 60
     250 end

## **CSV Program**

```

The following program will initialize and then execute continuously, repeating the same series of
operations; process events, read input from the serial port, write any processed data out to the ZPL port,
and then process the data read from the serial port.


The program first loads the CSV database E:PRODUCTS.CSV (in PROGRAMINIT subroutine). Then, data
read from the serial port is compared against the first column in the database. If an entry is found in the first
column of a row (in FINDITEM subroutine), the data for the respective row is inserted into the ZPL format
E:PRICELBL.ZPL and printed on a label.


**Example**


This is an example of a CVS program.

```
     1 REM SUBROUTINES BELOW....
     2 REM
     3 REM
     ************************************************************************
     4 REM        MAIN LOOP - DO NOT MODIFY
     5 REM
     ************************************************************************
     6 REM
     7 GOSUB PROGRAMINIT
     8 DO WHILE 1 = 1
     9 GOSUB PROCESSEVENTS
     10 GOSUB GETINPUT
     11 GOSUB WRITEOUTPUT
     12 GOSUB PROCESSDATA

```

587


ZBI Commands

```
13 LOOP
14 REM SUBROUTINES BELOW....
15 REM
16 REM
************************************************************************
17 REM        Program Init
18 REM
************************************************************************
19 REM
20 SUB PROGRAMINIT
21 LET INPORT = 1
22 LET OUTPORT = 2
23 LET ENDLINE$ = CHR$ ( 13 ) & CHR$ ( 10 )
24 OPEN # INPORT : NAME "SER"
25 OPEN # OUTPORT : NAME "ZPL"
26 DECLARE STRING DATABASE$ ( 1, 1 )
27 LET COLUMNCOUNT = CSVLOAD ( DATABASE$, "E:PRODUCTS.CSV" )
28 LET OUTDATA$ = "TABLE WITH " & STR$ ( COLUMNCOUNT ) & " COLUMNS LOADED" &
ENDLINE$
29 RETURN
30 REM
31 REM
************************************************************************
32 REM        Process Events
33 REM
************************************************************************
34 REM
35 SUB PROCESSEVENTS
36 RETURN
37 REM
38 REM
************************************************************************
39 REM        Get Input
40 REM
41 REM Writes All Data from the serial port to the string INDATA$
42 REM
************************************************************************
43 REM
44 SUB GETINPUT
45 IF LEN ( INDATA$ ) < 5000 THEN
46 LET INCOUNT = READ ( INPORT, A$, 1024 )
47 LET INDATA$ = INDATA$ & A$
48 END IF
49 RETURN
50 REM
51 REM
************************************************************************
52 REM        Write Output
53 REM
54 REM Writes All Data from the string OUTDATA$ to the ZPL Port
55 REM
************************************************************************
56 REM
57 SUB WRITEOUTPUT

```

588


ZBI Commands

```
58 LET OUTCOUNT = WRITE ( OUTPORT, OUTDATA$, LEN ( OUTDATA$ ) )
59 IF OUTCOUNT > 0 THEN
60 LET OUTDATA$ ( 1 : OUTCOUNT ) = ""
61 END IF
62 RETURN
63 REM
64 REM
***************************************************************************
65 REM        Process Data
66 REM
67 REM Parse the data in the string INDATA$ and write output to OUTDATA$
68 REM
************************************************************************
69 REM
70 SUB PROCESSDATA
71 IF LEN ( OUTDATA$ ) > 1000 THEN
72 RETURN
73 END IF
74 REM REMOVE ALL LINE FEEDS
75 DO
76 LET LOC = POS ( INDATA$, CHR$ ( 10 ) )
77 LET INDATA$ ( LOC : LOC ) = ""
78 LOOP WHILE LOC > 0
79 REM COMPLETED LINE FEED REMOVAL
80 LET LOC = POS ( INDATA$, CHR$ ( 13 ) ) ! Line ends with CR
81 IF LOC > 0 THEN
82 LET INLINE$ = INDATA$ ( 1 : LOC - 1 )
83 LET INDATA$ ( 1 : LOC ) = ""
84 GOSUB FINDITEM
85 IF ROW > 0 THEN
86 LET OUTDATA$ = OUTDATA$ & "^XA^XFE:PRICELBL.ZPL^FS" & ENDLINE$
87 LET OUTDATA$ = OUTDATA$ & "^FN1^FD" & DATABASE$ ( ROW, 1 ) & "^FS" &
ENDLINE$
88 LET OUTDATA$ = OUTDATA$ & "^FN2^FD" & DATABASE$ ( ROW, 2 ) & "^FS" &
ENDLINE$
89 LET OUTDATA$ = OUTDATA$ & "^FN3^FD" & DATABASE$ ( ROW, 3 ) & "^FS^XZ" &
ENDLINE$
90 END IF
91 END IF
92 RETURN
93 REM
94 REM
************************************************************************
95 REM        Find Item
96 REM
97 REM Search the first column of the database for the exact item requested
98 REM
************************************************************************
99 REM
100 SUB FINDITEM
101 LET ROW = 0
102 LET EXPECTED$ = INLINE$
103 DO
104 LET FOUNDENTRY$ = ""

```

589


ZBI Commands

```
     105 LET ROW = FIND ( DATABASE$, 1, EXPECTED$, ROW + 1 )
     106 IF ROW <> 0 THEN
     107 LET FOUNDENTRY$ = DATABASE$ ( ROW, 1 )
     108 END IF
     109 LOOP WHILE ( ROW <> 0 AND FOUNDENTRY$ <> EXPECTED$ )
     110 RETURN

## **DPI Conversion Program**

```

This program converts a ZPL format being sent to the printer on the parallel port to 300 dpi (dots per inch)
from 200 dpi (dots per inch). This is done by searching for and extracting ZPL commands with resolutiondependent arguments and scaling the arguments for a 300 dpi printer.


**Example**


This is an example of dpi conversion:

```
     1 rem **************************************************
     1 rem Zebra Technologies ZBI Sample Program
     1 rem
     1 rem Professional programming services are available. Please contact
     1 rem ZBI-Experts@zebra.com for more information.
     1 rem
     1 rem This is an example of converting a printer from 200 dpi (dots
     1 rem per inch
     1 rem to 300 dpi. This example covers only some of the ZPL commands
     1 rem that
     1 rem could be affected by converting from 200 to 300 dpi printing.
     1 rem **************************************************
     1 rem open the ports for input and output
     1 rem **************************************************
     10 close #1
     20 close #2
     30 open #1 : name "PAR"
     40 open #2 : name "ZPL"
     1 rem **************************************************
     1 rem create an array with the search parameters
     1 rem **************************************************
     50 declare string find$(20)
     60 let find$(1) = "^FO"
     70 let find$(2) = "^A0"
     80 let find$(3) = "^GB"
     90 let find$(4) = "^XZ"
     100 let find$(5) = "^A@"
     110 let find$(6) = "^LL"
     120 let find$(7) = "^LH"
     130 let find$(8) = "FO"
     140 let find$(9) = "A0"
     150 let find$(10) = "GB"
     160 let find$(11) = "XZ"
     170 let find$(12) = "A@"
     180 let find$(14) = "LH"
     190 let find$(15) = "^BY"

```

590




ZBI Commands

```
200 let find$(16) = "BY"
210 let find$(17) = "^B3"
220 let find$(18) = "B3"
1 rem *******************************************************
1 rem search for the parameters
1 rem *******************************************************
300 do
310  let in$ = searchto$(1, find$, 2)
1 rem ********************************************************
1 rem once a parameter is found, determine how to handle it
1 rem ********************************************************
320  if in$ = "^FO" or in$ = "FO" then
330    gosub 520
340  else if in$ = "^LH" or in$ = "LH" then
350    gosub 520
360  else if in$ = "^A0" or in$ = "A0" then
370    gosub 700
380  else if in$ = "^A@" or in$ = "A@" then
390    gosub 700
400  else if in$ = "^GB" or in$ = "GB" then
410    gosub 1100
420  else if in$ = "^LL" then
430    gosub 1300
440  else if in$ = "^BY" or in$ = "BY" then
450    gosub 1400
460  else if in$ = "^B3" or in$ = "B3" then
470    gosub 1600
480  else if in$ = "^XZ" then
490    print #2: in$;
500  end if
510 loop
1 rem ********************************************************
1 rem convert the ^FO and ^LH commands from 200 to 300 dpi
1 rem ********************************************************
520 inbyte #1: a$
530 let a = ord(a$)
540 if a >= 65 then
550  print #2: in$&a$;
560  goto 660
570 end if
580 let x$ = extract$(1, "", ", ")
590 let x2$ = a$&x$
600 let y$ = extract$(1, "", "^")
610 let x = val(x2$)
620 let y = val(y$)
630 let x2 = (x/2)+x
640 let y2 = (y/2)+y
650 print #2: in$; x2; ","; y2; "^";
660 return
1 rem ********************************************************
1 rem convert the ^A0 and ^A@ commands from 200 to 300 dpi
1 rem ********************************************************
700 inbyte #1: a$
710 let a = ord(a$)

```

591


ZBI Commands

```
720 let b = 0
730 let c = 0
740 if a >= 65 then
750  print #2: in$&a$; ",";
760  let b = 1
770 end if
780 inbyte #1: a$
790 let h$ = extract$(1, "", ",")
800 if in$ = "^A@" or in$ = "A@" then
810  let c = 1
820  let w$ = extract$(1, "", ",")
830  let m$ = extract$(1, "", "^")
840 else
850  let w$ = extract$(1, "", "^")
860 end if
870 let h = val(h$)
880 let w = val(w$)
900 let h2 = (h/2) + h
910 let w2 = (w/2) + w
920 if b = 1 then
930  print #2: h2; ","; w2;
940 else
950  print #2: in$&"N,"; h2; ","; w2;
960 end if
970 if c = 1 then
980  print #2: ","; m$;
990 end if
1000 print #2: "^";
1010 return
1 rem ********************************************************
1 rem convert the ^GB command from 200 to 300 dpi
1 rem ********************************************************
1020 let w$ = extract$(1, "", ",")
1030 let h$ = extract$(1, "", ",")
1040 let t$ = extract$(1, "", "^")
1050 let h = val(h$)
1060 let w = val(w$)
1070 let t = val(t$)
1080 let h2 = (h/2)+ h
1090 let w2 = (w/2)+ w
1100 let t2 = (t/2)+ t
1110 print #2: in$; w2; ","; h2; ","; t2; "^";
1120 return
1 rem ********************************************************
1 rem convert the ^LL command from 200 to 300 dpi
1 rem ********************************************************
1300 let l$ = extract$(1, "", "^")
1310 let l = VAL(l$)
1320 let l2 = (l/2) + l
1330 print #2: in$; l2; "^";
1340 return
1 rem ********************************************************
1 rem convert the ^BY command from 200 to 300 dpi
1 rem ********************************************************

```

592


ZBI Commands

```
     1400 inbyte #1: a$
     1410 let a = ord(a$)
     1420 if a >= 48 and a <= 57 then
     1460  let x$ = extract$(1, "", ", ")
     1470  let x2$ = a$&x$
     1480  let x = val(x2$)
     1490  let x2 = (x/2) + x
     1500    if x2 > 10 then
     1510    let x2 = 10
     1520    end if
     1530  print #2: in$; x2; ",";
     1540 else
     1550  print #2: in$; a$;
     1560 end if
     1570 return
     1 rem ********************************************************
     1 rem convert the ^B3 command from 200 to 300 dpi
     1 rem ********************************************************
     1600 let o$ = extract$(1, "", ", ")
     1610 let e$ = extract$(1, "", ", ")
     1620 let h$ = extract$(1, "", ", ")
     1630 let h = val(h$)
     1640 let h2 = (h/2) + h
     1650 print #2: in$; o$; ","; e$; ","; h2; ",";
     1660 return