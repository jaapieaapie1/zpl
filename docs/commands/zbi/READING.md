# Reading and Writing



This manual has detailed various functions to read and write to all of the ports. The following section gives
an overview of the commands, functions, and when each should be used.


To start, it is important to understand the term "blocking". In communications code, a function or command
is "blocking" if it waits for all of the requested data to be received before it returns.


**INPUT (blocking)**
Reads one line into each string specified.

**PRINT (blocking)**
Simple method to write specified expressions out.

**OUTBYTE (blocking)**
Writes one byte out.

**INBYTE (blocking)**
Reads in one byte.

**READ (non-blocking)**
Reads in all available data up to the maximum amount specified.

**WRITE (non-blocking)**
Writes out as much data as possible up to a maximum specified amount.

**SEARCHTO$ (blocking)**
Reads in data (does not keep) until a search parameter is found. Non-matching data can be
redirected to another port.


493