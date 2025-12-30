# Running and Debugging Commands



The following commands were written before the development of the ZBI-Developer application. With that
application, and when using ZBI version 1, the following commands are essentially obsolete. However,
for those who started developing ZBI applications before the following reference will be
ZBI-Developer,
helpful.


**RUN**
Starts executing the program currently in memory at the first line of the program

**CTRL-C**
Sends an end-of-transmission character, `ETX`, to the console to terminate the ZBI program
currently running.

**RESTART**
Starts executing the program currently in memory where it was last stopped

**STEP**
Executes one line of the program in memory where it was last stopped

**DEBUG**
This mode controls whether or not the TRACE and BREAK commands are processed

**TRACE**
Shows which lines have been executed and which variables have been changed

**BREAK**
Stops the currently running program

**ADDBREAK**
Adds a break to an existing line

**DELBREAK**
Deletes an existing break

**ZPL**
Terminates and exits the ZBI environment.


455


ZBI Commands


**Example**
This example shows many of the Running and Debug Commands in practice.


0 PRINT "TEN"
20 PRINT "TWENTY"
30 PRINT "THIRTY"
RUN
TEN
TWENTY
THIRTY


STEP
TEN


RESTART
TWENTY
THIRTY


ADDBREAK 20
RUN
TEN
<Program Break> on line: 20


DEBUG ON
TRACE ON
RESTART
<TRACE> 20
TWENTY
<TRACE> 30
THIRTY


456