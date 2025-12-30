# Events




ZBI Commands


This section explains how to capture and trigger internal events in the printer. Here is a quick list of these
commands:


**Available Events**
A table that correlates a ZBI event with an identification number.

**ZBI Key Names**
Details the names of each printer’s control panel buttons, ZBI names, and ZBI event ID.

**REGISTEREVENT**
Sets up the `HANDLEEVENT` function to receive notification when the specified event has occurred.

**UNREGISTEREVENT**
Allows events that are currently set to be captured by the program to no longer be captured.

**HANDLEEVENT**
Once events have been registered, this function is used to see what events have occurred.

**TRIGGEREVENT**
Allows for control panel buttons to be triggered programmatically.


There are certain events in the printer that a ZBI 2.0 program can receive. To do this, the program
first registers for the event. On a regular basis, call a function to handle events. When an event
occurs that the program is registered for, the function will return the event’s identification number.


**Available Events**

|ZBI Event ID|ZBI Event|
|---|---|
|1|`menu key`|
|2|`pause key`|
|3|`feed key`|
|4|`cancel key`|
|5|`up arrow key`|
|6|`plus key`|
|7|`minus key`|
|8|`enter key`|
|9|`setup exit key`|
|10|`select key`|
|11|`cancel all event`|
|12|`config label`|
|13|`timer1`|
|14|`timer2`|
|15|`timer3`|
|16|`timer4`|
|17|`timer5`|
|18|`spare unused`|
|19|`previous key`|



520




ZBI Commands

|ZBI Event ID|ZBI Event|
|---|---|
|20|`next save key`|
|21|`calibrate key`|
|22|`paper out set`|
|23|`paper out clear`|
|24|`ribbon out set`|
|25|`ribbon out clear`|
|26|`head too hot set`|
|27|`head too hot clear`|
|28|`head cold set`|
|29|`head cold clear`|
|30|`head open set`|
|31|`head open clear`|
|32|`supply too hot set`|
|33|`supply too hot clear`|
|34|`ribbon in set`|
|35|`ribbon in clear`|
|36|`rewind full set`|
|37|`rewind full clear`|
|38|`cutter jammed set`|
|39|`cutter jammed clear`|
|40|`paused set`|
|41|`paused clear`|
|42|`pq completed set`|
|43|`pq completed clear`|
|44|`label ready set`|
|45|`label ready clear`|
|46|`head element bad set`|
|47|`head element bad clear`|
|48|`basic runtime set`|
|49|`basic runtime clear`|
|50|`basic forced set`|
|51|`basic forced clear`|
|52|`power on set`|
|53|`power on clear`|
|54|`clean printhead set`|



521


ZBI Commands

|ZBI Event ID|ZBI Event|
|---|---|
|55|`clean printhead clear`|
|56|`media low set`|
|57|`media low clear`|
|58|`ribbon low set`|
|59|`ribbon low clear`|
|60|`replace head set`|
|61|`replace head clear`|
|62|`battery low set`|
|63|`battery low clear`|
|64|`rfid error set`|
|65|`rfid error clear`|
|66|`any messages set`|
|67|`any messages clear`|
|68|`auto baud`|
|69|`factory default`|
|70|`networking default`|
|71|`networking factory`|
|72|`print width`|
|73|`darkness adjust`|
|74|`calibrate`|
|75|`scroll key`|
|76|`soft key 1`|
|77|`soft key 2`|
|78|`ribbon cartridge authentication error set`|
|79|`ribbon cartridge authentication error clear`|


## **ZBI Key Names**


This section details the names to use for each printer’s front panel buttons when creating ZBI 2.0 programs
to capture the buttons.


**ZT200/ZT400/ZT500/ZT600/ZD500/Qln**









|ZT2X0|ZT400/<br>ZT500/<br>ZT600|ZD500|Qln|ZBI<br>Event ID|ZBI Name|
|---|---|---|---|---|---|
|Left Soft button|Left Soft button|Left Soft button|Left Soft button|76|`soft key 1`|
|Right Soft Button|Right Soft Button|Right Soft Button|Right Soft Button|77|`soft key 2`|
|Plus|Up Arrow|Up Arrow|Up Arrow|6|`plus key`|


522


ZBI Commands





|ZT2X0|ZT400/<br>ZT500/<br>ZT600|ZD500|Qln|ZBI<br>Event ID|ZBI Name|
|---|---|---|---|---|---|
|Minus|Down Arrow|Down Arrow|Down Arrow|7|`minus key`|
|Left Arrow|Left Arrow|Left Arrow|Left Arrow|19|`previous key`|
|Right Arrow|Right Arrow|Right Arrow|Right Arrow|20|`next save key`|
|Setup|OK|Check|OK|10|`select key`|
|Pause|Pause|Pause|no key|2|`pause key`|
|Feed|Feed|Feed|Feed|3|`feed key`|
|Cancel|Cancel|Cancel|no key|4|cancel key|


**Xi4/RXi4/XiIIIPlus/PAX4/105SL/ZE500**











|XiIIIPlus/PAX4/Xi4/<br>RXi4/ ZE500/105SL<br>Plus Front Panel Key|105SL Front<br>Panel Key|ZBI<br>Event ID|ZBI Name|
|---|---|---|---|
|Right Oval|Plus (+)|6|`plus key`|
|Left Oval|Minus (-)|7|`minus key`|
|Previous|Previous|19|`previous key`|
|Next/Save|Next/Save|20|`next save key`|
|Setup/Exit|Setup/Exit|9|`setup exit key`|
|Pause|Pause|2|`pause key`|
|Feed|Feed|3|`feed key`|
|Cancel|Cancel|4|`cancel key`|
|Calibrate|Calibrate|21|`calibrate key`|


**HC100**

|Front Panel Key|ZBI Event ID|ZBI Name|
|---|---|---|
|Pause|2|`pause key`|
|Feed|3|`feed key`|
|Eject||`eject key`|



**ZM400/ZM600/RZ400/RZ600/Z4Mplus/Z6Mplus**

|Front Panel Key|ZBI Event ID|ZBI Name|
|---|---|---|
|Feed|3|`feed key`|
|Pause|2|`pause key`|
|Cancel|4|`cancel key`|
|Setup/Exit|9|`setup exit key`|
|Select|10|`select key`|
|Plus (+)|6|`plus key`|



523


ZBI Commands

|Front Panel Key|ZBI Event ID|ZBI Name|
|---|---|---|
|Minus (-)|7|`minus key`|



**S4M**

|Front Panel Key|ZBI Event ID|ZBI Name|
|---|---|---|
|Menu|1|`menu key`|
|Enter|8|`enter key`|
|Cancel|4|`cancel key`|
|Feed|3|`feed key`|
|Pause|2|`pause key`|
|Left Arrow|4|`cancel key`|
|Right Arrow|3|`feed key`|
|Up Arrow|5|`up arrow key`|
|Down Arrow|2|`pause key`|



**G-Series**

|Front Panel Key|ZBI Event ID|ZBI Name|
|---|---|---|
|Feed key|3|`Feed key`|
|Select key|10|`Select key`|
|Scroll key|75|`Scroll key`|



**KR403 / 2824 Plus Series**

|Front Panel Key|ZBI Event ID|ZBI Name|
|---|---|---|
|Feed key|3|`Feed key`|



524


ZBI Commands