# Extraction 2 Program


```

This program finds and stores data of interest, which in this case is found in a format after the string "DATA
= ". The input command is used to get the data from the input stream, and it is inserted into a simple ZPL
format to be printed.


**Example**


This is an example of Extraction 2.


595


ZBI Commands

```
1 rem******************************************************
1 rem Zebra Technologies ZBI Sample Program
1 rem
1 rem Professional programming services are available. Please contact
1 rem ZBI-Experts@zebra.com for more information.
1 rem
1 rem This is an example of using ZBI for data extraction.
1 rem There are two methods for doing extraction; this example shows
1 rem data extraction from the port directly.
1 rem
1 rem The data to extract is as follows:
1 rem START
1 rem DATA = "hello":
1 rem DATA = "goodbye":
1 rem END
1 rem******************************************************
1 rem close ports except console, open channels to parallel and serial ports
1 rem******************************************************
05 for i = 1 to 9 step 1
10  close #i
20 next i
30 open #1: name "PAR"
40 open #2: name "ZPL"
1 rem******************************************************
1 rem create string array to hold data
1 rem*******quotes and print to ZPL port with formatting
1 rem***************************************************

50 declare string format$(3)
60 let format$(1) = "START"
70 let format$(2) = "END"
80 let format$(3) = "DATA"
1 rem******************************************************
1 rem main program; look for "START" keyword, if found print ^XA to ZPL port
1 rem******************************************************

90 do
100  let begin$ = searchto$(1, format$, 2)
110  if begin$ = "START" then
120    print #2: "^XA";
1 rem******************************************************
1 rem if "DATA" keyword is found, get two data strings
1 rem******************************************************
130  else if begin$ = "DATA" then
1 rem***************************************************
1 rem get data from between q
140    let extracted_data1$ = extract$(1,"""","""")
150    input #1: junk$
170    let extracted_data2$ = extract$(1,"""","""")
180    print #2:"^FO30,30^A0N,30,30^FD" &extracted_data1$& "^FS";
190    print #2:"^FO30,70^A0N,30,30^FD" &extracted_data2$& "^FS";
200  else if begin$ = "END" then
210    print #2: "^XZ"

```

596


ZBI Commands

```
     220  end if
     230 loop