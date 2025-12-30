# ^JH




ZPL Commands


The `^JH` command configures the early warning messages that appear on the LCD.


**Early Warning Settings**


- ZE500 series


- **Xi** III, **Xi** III **Plus**, Xi4, RXi4


- **PAX** 3, **PAX** 4


- ZM400, ZM600, RZ400, RZ600


- S4M


- G-Series (“f” parameter only)

**Format:** `^JHa,b,c,d,e,f,g,h,i,j`







|Parameter|Details|
|---|---|
|`a =` early warning<br>media<br>`a` = supplies warning<br>(Xi4 and RXi4 printers<br>only)|This parameter is for XiIIIPlus, Xi4, RXi4, PAX3, and PAX4 printers only.<br>**Values:**<br>`E =` enable<br>`D =` disable<br>**Default:** `D`|
|`b =` labels per roll|This parameter is for XiIIIPlus, PAX3, and PAX4 printers only.<br>**Values:** `100` to`9999`<br>**Default:** `900`|
|`c =` media replaced|This parameter is for XiIIIPlus, PAX3, and PAX4 printers only.<br>**Values:**<br>`Y =` yes<br>`N =` no<br>**Default:** `N`|


259


ZPL Commands








|Parameter|Details|
|---|---|
|`d =` ribbon length|This parameter is for XiIIIPlus, PAX3, PAX4, and ZE500 printers only.<br>**Values:**<br>XiIIIPlus series printers:<br>`N = 0M`<br>`0 =` 100M<br>`1 =` 150M<br>`2 =` 200M<br>`3 =` 250M<br>`4 =` 300M<br>`5 =` 350M<br>`6 =` 400M<br>`7 =` 450M<br>PAX series printers:<br>`N = 0M`<br>`0 =` 100M<br>`1 =` 150M<br>`2 =` 200M<br>`3 =` 250M<br>`4 =` 300M<br>`5 =` 350M<br>`6 =` 400M<br>`7 =` 450M<br>`10 =` 600M<br>`11 =` 650M<br>`12 =` 700M<br>`13 =` 750M<br>`14 =` 800M<br>`15 =` 850M<br>`16 =` 900M<br>ZE500 series printers:<br>`N = 0M`<br>`0 =` 100M<br>`1 =` 150M<br>`2 =` 200M<br>`3 =` 250M<br>`4 =` 300M<br>`5 =` 350M<br>`6 =` 400M<br>`7 =` 450M<br>`10 =` 600M<br>**Default:**<br>1 - for 96XiIIIPlus<br>7 - for all other printers<br>260|




ZPL Commands





|Parameter|Details|
|---|---|
|`e =` ribbon replaced|This parameter is for**Xi**III**Plus**, **PAX**3, and**PAX**4 printers only.<br>**Values:**<br>`Y =` yes<br>`N =` no<br>**Default:** `N`|
|`f =` early warning<br>maintenance|This parameter is for Xi4, RXi4,**PAX**4, ZM400, ZM600, RZ400, RZ600, and S4M<br>printers only.<br>**Values:**<br>`E =` enabled<br>`D =` disabled<br>**Default:** `D`<br>**IMPORTANT:** On G-Series printers, this parameter must be enabled<br>for the`^MA` driven system to work.|
|`g =` head cleaning<br>interval|Accepted value exceptions: accepted values for**Xi**III printer are 100M through<br>450M; accepted values for 600 dpi**Xi**III printers are 100M through 150M;<br>accepted values for**PAX**4 series printers are up to 900M by increments of 50M;<br>accepted values for ZM400/ZM600, RZ400/RZ600, and S4M printers are 0M<br>through 450M.<br>**Values:**<br>`0 =` 100M<br>`1 =` 150M<br>`2 =` 200M<br>`3 =` 250M<br>`4 =` 300M<br>`5 =` 350M<br>`6 =` 400M<br>`7 =` 450M<br>`8 =` 500M<br>`9 =` 550M<br>`10=` 600M<br>`11 =` 650M<br>`12 =` 700M<br>`13 =` 750M<br>`14 =` 800M<br>`15 =` 850M<br>`16 =` 900M<br>**Default:**<br>1 - for 96XiIIIPlus<br>7 - for all other printers|


261


ZPL Commands







|Parameter|Details|
|---|---|
|`h =` head clean|**Values:**<br>`N =` No<br>`Y =` Yes<br>**Default:**`N`|
|`i =` head life<br>threshold|**Values:**<br>0 – 0 in or off100-3500000 in**Default:** 1000000|
|`j =` head replaced|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|


**Comments:** To permanently save the changes to the `^JH` command, send `^XA^JUS^XZ` .


262