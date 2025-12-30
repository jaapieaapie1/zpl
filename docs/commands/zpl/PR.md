# ^PR




ZPL Commands


The `^PR` command determines the media and slew speed (feeding a blank label) during printing.


**Print Rate**


The printer operates with the selected speeds until the setting is reissued or the printer is turned off.


The print speed is application-specific. Because print quality is affected by media, ribbon, printing speeds,
and printer operating modes, it is very important to run tests for your applications.


**NOTE:** important: Some models go to default print speed when power is turned off.

```
^PRp,s,b

|Parameters|Details|
|---|---|
|`p =` print speed|**Values:**<br>1  =  25.4 mm/sec. (1 inch/sec.)1<br>A or 2  =  50.8 mm/sec. (2 inches/sec.)<br>B or 3  =  76.2 mm/sec. (3 inches/sec.)<br>C or 4  =  101.6 mm/sec. (4 inches/sec.)<br>5  =  127 mm/sec.(5 inches/sec.)<br>D or 6  =  152.4 mm/sec. (6 inches/sec.)<br>7  =  177.8 mm/sec. (7 inches/sec.)<br>E or 8  =  203.2 mm/sec. (8 inches/sec.)<br>9  =  220.5 mm/sec. 9 inches/sec.)<br>10  =  245 mm/sec.(10 inches/sec.)<br>11  =  269.5 mm/sec.(11 inches/sec.)<br>12  =  304.8 mm/sec. 12 inches/sec.)<br>13  =  13 in/sec2<br>14  =  14 in/sec2<br>**Default:** `A`|


```

325


ZPL Commands

|Parameters|Details|
|---|---|
|`s =` slew speed|**Values:**<br>A or 2  =  50.8 mm/sec. (2 inches/sec.)<br>B or 3  =  76.2 mm/sec. (3 inches/sec.)<br>C or 4  =  101.6 mm/sec. (4 inches/sec.)<br>5  =  127 mm/sec. 5 inches/sec.)<br>D or 6  =  152.4 mm/sec. (6 inches/sec.)<br>7  =  177.8 mm/sec. (7 inches/sec.)<br>E or 8  =  203.2 mm/sec. (8 inches/sec.)<br>9  =  220.5 mm/sec. (9 inches/sec.)<br>10  =  245 mm/sec. (10 inches/sec.)<br>11  =  269.5 mm/sec. 11 inches/sec.)<br>12  =  304.8 mm/sec. 12 inches/sec.)<br>13  =  13 in/sec2<br>14  =  14 in/sec2<br>**Default:** `D`|
|`b =` backfeed speed|**Values:**<br>A or 2  =  50.8 mm/sec. (2 inches/sec.)<br>B or 3  =  76.2 mm/sec. (3 inches/sec.)<br>C or 4  =  101.6 mm/sec. (4 inches/sec.)<br>5  =  127 mm/sec.(5 inches/sec.)<br>D or 6  =  152.4 mm/sec. (6 inches/sec.)<br>7  =  177.8 mm/sec. (7 inches/sec.)<br>E or 8  =  203.2 mm/sec. (8 inches/sec.)<br>9  =  220.5 mm/sec. 9 inches/sec.)<br>10  =  245 mm/sec. 10 inches/sec.)<br>11  =  269.5 mm/sec. 11 inches/sec.)<br>12  =  304.8 mm/sec. 12 inches/sec.)<br>13  =  13 in/sec14  =  14 in/sec2<br>**Default:** `A`|



1. This value is supported only on the 110Xi4-600dpi, 110XiIIIPlus-600dpi, and RXi printers.


2. This value is supported only on the Xi4 and RXi4 printers.

**Comments:** The speed setting for `p`, `s`, and `b` is dependent on the limitations of the printer. If a particular
printer is limited to a rate of 6 ips (inches per second), a value of 12 can be entered but the printer performs
only at a 6 ips rate. See your printer’s User Guide for specifics on performance.


This command is ignored on the HC100 printer.


326