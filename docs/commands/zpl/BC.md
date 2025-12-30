# ^BC




ZPL Commands


The `^BC` command creates the Code 128 barcode, a high-density, variable length, continuous,
alphanumeric symbology. It was designed for complexly encoded product identification.


**Code 128 Barcode (Subsets A, B, and C)**


Code 128 has three subsets of characters. There are 106 encoded printing characters in each set, and each
character can have up to three different meanings, depending on the character subset being used. Each
Code 128 character consists of six elements: three bars and three spaces.

- `^BC` supports a fixed print ratio.

- Field data ( `^FD` ) is limited to the width (or length, if rotated) of the label.


**IMPORTANT:** [If additional information about this barcode is required, refer to aimglobal.org.](http://www.aimglobal.org)


**Format:** `^BCo,h,f,g,e,m`







|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` barcode height (in<br>dots)|**Values:** `1` to`32000`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Values:** `Y` (yes) or`N` (no)<br>**Default:** `Y`<br>The interpretation line can be printed in any font by placing the font<br>command before the barcode command.|
|`g =` print interpretation<br>line above code|**Values:** `Y` (yes) or`N` (no)<br>**Default:** `N`|
|`e =` UCC check digit|**Values:** `Y` (turns on) or`N` (turns off)<br>Mod 103 check digit is always there. It cannot be turned on or off. Mod 10<br>and 103 appear together with`e` turned on.<br>**Default:** `N`|


94


ZPL Commands

|Parameters|Details|
|---|---|
|`m =` mode|**Values:**<br>`N =` no selected mode<br>`U =` UCC Case Mode<br>•<br>More than 19 digits in`^FD` or`^SN` are eliminated.<br>•<br>Fewer than 19 digits in`^FD` or ^SN add zeros to the right to bring the<br>count to 19. This produces an invalid interpretation line.<br>`A =`  Automatic Mode<br>This analyzes the data sent and automatically determines the best packing<br>method. The full ASCII character set can be used in the`^FD` statement<br>— the printer determines when to shift subsets. A string of four or more<br>numeric digits causes an automatic shift to Subset C.<br>`D =` UCC/EAN Mode (x.11.x and newer firmware)<br>This allows dealing with UCC/EAN with and without chained application<br>identifiers. The code starts in the appropriate subset followed by FNC1<br>to indicate a UCC/EAN 128 barcode. The printer automatically strips out<br>parentheses and spaces for encoding but prints them in the human-<br>readable section. The printer automatically determines if a check digit is<br>required, calculates it, and prints it. Automatically sizes the human readable.<br>**Default:** N|



**Example:** This is an example of a Code 128 barcode:


**Figure 2** Code 128 Barcode


**Code 128 Subsets**


The Code 128 character subsets are referred to as Subset A, Subset B, and Subset C. A subset can be
selected in these ways:

- A special Invocation Code can be included in the field data ( `^FD` ) string associated with that barcode.

- The desired Start Code can be placed at the beginning of the field data. If no Start Code is entered,
Subset B is used.


To change subsets within a barcode, place the Invocation Code at the appropriate points within the field
data ( `^FD` ) string. The new subset stays in effect until changed with the Invocation Code. For example, in
Subset C, `>7` in the field data changes the Subset to A.

The following table shows the Code 128 Invocation Codes and Start Characters for the three subsets.


95


ZPL Commands


**Table 2** Code 128 Invocation Characters







|Invocation Code|Decimal Value|Subset A<br>Character|Subset B<br>Character|Subset C<br>Character|
|---|---|---|---|---|
|><|62||||
|>0|30|>|>||
|>=|94||~||
|>1|95|USQ|DEL||
|>2|96|FNC 3|FNC 3||
|>3|97|FNC 2|FNC 2||
|>4|98|SHIFT|SHIFT||
|>5|99|CODE C|CODE C||
|>6|100|CODE B|FNC 4|CODE B|
|>7|101|FNC 4|CODE A|CODE A|
|>8|102|FNC 1|FNC 1|FNC 1|
|**Start Characters**|**Start Characters**|**Start Characters**|**Start Characters**|**Start Characters**|
|>9|103|Start CodeA|(Numeric Pairs<br>give Alpha/<br>Numerics)||
|>:|104|Start CodeB|(Normal Alpha/<br>Numeric)||
|>;|105|Stat CodeC|(All Numeric(00 -<br>99)||


The following table shows the character sets for Code 128:


**Table 3** Code 128 Character Sets

|Value|CodeA|CodeB|CodeC|Col5|Value|CodeA|CodeB|CodeC|
|---|---|---|---|---|---|---|---|---|
|0|SP|SP|00||53|U|U|53|
|1|!|!|01||54|V|V|54|
|2|''|''|02||55|W|W|55|
|3|#|#|03||56|X|X|56|
|4|$|$|04||57|Y|Y|57|
|5|%|%|05||58|Z|Z|58|
|6|&|&|06||59|[|[|59|
|7|'|'|07||60|\|\|60|
|8|(|(|08||61|]|]|61|
|9|)|)|09||62|^|^|62|
|10|*|*|10||63|_|_|63|



96


ZPL Commands


**Table 3** Code 128 Character Sets (Continued)

|Value|CodeA|CodeB|CodeC|Col5|Value|CodeA|CodeB|CodeC|
|---|---|---|---|---|---|---|---|---|
|11|+|+|11||64|NUL|.|64|
|12|,|,|12||65|SOH|a|65|
|13|-|-|13||66|STX|b|66|
|14|.|.|14||67|ETX|c|67|
|15|/|/|15||68|EOT|d|68|
|16|0|0|16||69|ENQ|e|69|
|17|1|1|17||70|ACK|f|70|
|18|2|2|18||71|BEL|g|71|
|19|3|3|19||72|BS|h|72|
|20|4|4|20||73|HT|i|73|
|21|5|5|21||74|LF|j|74|
|22|6|6|22||75|VT|k|75|
|23|7|7|23||76|FF|l|76|
|24|8|8|24||77|CR|m|77|
|25|9|9|25||78|SO|n|78|
|26|:|:|26||79|SI|o|79|
|27|;|;|27||80|DLE|p|80|
|28|<|<|28||81|DC1|q|81|
|29|=|=|29||82|DC2|r|82|
|30|>|>|30||83|DC3|s|83|
|31|?|?|31||84|DC4|t|84|
|32|@|@|32||85|NAK|u|85|
|33|A|A|33||86|SYN|v|86|
|34|B|B|34||87|ETB|w|87|
|35|C|C|35||88|CAN|x|88|
|36|D|D|36||89|EM|y|89|
|37|E|E|37||90|SUB|z|90|
|38|F|F|38||91|ESC|{|91|
|39|G|G|39||92|FS|||92|
|40|H|H|40||93|GS|}|93|
|41|I|I|41||94|RS|~|94|
|42|J|J|42||95|US|DEL|95|
|43|K|K|43||96|FNC3|FNC3|96|



97


ZPL Commands


**Table 3** Code 128 Character Sets (Continued)

|Value|CodeA|CodeB|CodeC|Col5|Value|CodeA|CodeB|CodeC|
|---|---|---|---|---|---|---|---|---|
|44|L|L|44||97|FNC2|FNC2|97|
|45|M|M|45||98|SHIFT|SHIFT|98|
|46|N|N|46||99|CodeC|CodeC|99|
|47|O|O|47||100|CodeB|FNC4|CodeB|
|48|P|P|48||101|FNC4|CodeA|CodeA|
|49|Q|Q|49||102|FNC1|FNC1|FNC1|
|50|R|R|50||103||START<br>(CodeA)||
|51|S|S|51||104||START<br>(CodeB)||
|52|T|T|52||105||START<br>(CodeC)||



**Example:** The following figures are examples of identical barcodes.


**Figure 3** Subset B with no Start Character


**Figure 4** Subset B with Start Character


**Example:** Because Code 128 Subset B is the most commonly used subset, ZPL II defaults to Subset B if no
start character is specified in the data string.


This figure is an example of switching from Subset C to B to A.


**Figure 5** Switching from Subset C to B to A


98


ZPL Commands


**How ^BC Works Within a ZPL II Script**

`^XA` - the first command starts the label format.

`^FO100,75` - the second command sets the field origin at 100 dots across the x-axis and 75 dots down
the y-axis from the upper-left corner.

`^BCN,100,Y,N,N` - the third command calls for a Code 128 barcode to be printed with no rotation (N)
and a height of 100 dots. An interpretation line is printed (Y) below the barcode (N). No UCC check digit is
used (N).

`^FDCODE128^FS` (Figure A) `^FD>:CODE128^FS` (Figure B) – the field data command specifies the content
of the barcode.

`^XZ` - the last command ends the field data and indicates the end of the label.

The interpretation line prints below the code with the UCC check digit turned off.

The `^FD` command for Figure A does not specify any subset, so Subset B is used. In Figure B, the `^FD`
command specifically calls Subset B with the >: Start Code. Although ZPL II defaults to Code B, it is good
practice to include the Invocation Codes in the command.


Code 128 – Subset B is programmed directly as ASCII text, except for values greater than 94 decimal and a
few special characters that must be programmed using the invocation codes. Those characters are:

```
^ > ~

```

**Code 128 – Subsets A and C**


Code 128, Subsets A and C are programmed in pairs of digits, 00 to 99, in the field data string. For details,
see Table 2  Code 128 Invocation Characters on page 96.


In Subset A, each pair of digits results in a single character being encoded in the barcode; in Subset C,
characters are printed as entered. Figure E below is an example of Subset A (>9 is the Start Code for
Subset A).


Nonintegers programmed as the first character of a digit pair (D2) are ignored. However, nonintegers
programmed as the second character of a digit pair (2D) invalidate the entire digit pair, and the pair is
ignored. An extra unpaired digit in the field data string just before a code shift is also ignored.


The figures below are examples of Subset C. Notice that the barcodes are identical.


**Figure 6** Subset C with Normal Data


**Figure 7** Subset C with Ignored Alpha Character


99


ZPL Commands


In the program code for the following figure, the D is ignored, and the 2 is paired with the 4.


**Figure 8** Subset A


**The UCC/EAN-128 Symbology**


The symbology specified for the representation of Application Identifier data is UCC/EAN-128, a variant of
Code 128, exclusively reserved for EAN International and the Uniform Code Council (UCC).


**NOTE:** It is not intended to be used for data to be scanned at the point of sale in retail outlets.


UCC/EAN-128 offers several advantages. It is one of the most complete, alphanumeric, one-dimensional
symbologies available today. The use of three different character sets (A, B, and C), facilitates the encoding
of the full 128 ASCII character set. Code 128 is one of the most compact linear barcode symbologies.
Character set C enables numeric data to be represented in a double-density mode. In this mode, two
digits are represented by only one symbol character saving valuable space. The code is concatenated.
That means that multiple AIs and their fields may be combined into a single barcode. The code is also
very reliable. Code 128 symbols use two independent self-checking features which improves printing and
scanning reliability.


UCC/EAN-128 barcodes always contain a special non-data character known as function 1 (FNC 1),
which follows the start character of the barcode. It enables scanners and processing software to autodiscriminate between UCC/EAN-128 and other barcode symbologies, and subsequently only process
relevant data.


The UCC/EAN-128 barcode is made up of a leading quiet zone, a Code 128 start character A, B, or C, an
FNC 1 character, Data (Application Identifier plus data field), a symbol check character, a stop character,
and a trailing quiet zone.


UCC/EAN and UCC/128 are a couple of ways you'll hear someone refer to the code. This just indicates that
the code is structured as dictated by the application identifiers that are used.


SSCC (Serial Shipping Container Code) formatted following the data structure layout for Application
Identifier 00. See Table 4  UCC/EAN Application Identifier on page 103. It could be 00 which is the
SSCC code. The customer needs to let us know what application identifiers are used for their barcode so
we can help them.


There are several ways of writing the code to print the code to the Application Identifier '00' structure.


**Using N for the Mode (m) Parameter**


This example shows with application identifier 00 structure:


100




ZPL Commands


**Figure 9** N for the M Parameter


- >;>8' sets it to subset C, function 1


- '00' is the application identifier followed by '17 characters', the check digit is selected using the 'Y' for
the (e) parameter to automatically print the 20th character.


- you are not limited to 19 characters with the mode set to N


**Using U for the Mode (m) Parameter**


The example shows the application identifier 00 format:


**Figure 10** U for the M Parameter


UCC Case Mode

- Choosing `U` selects UCC Case mode. You will have exactly 19 characters available in `^FD` .

- Subset C using FNC1 values are automatically selected.


- Check digit is automatically inserted.


**Using D for the Mode (m) Parameter**


This example shows application identifier 00 format ((x.11.x or later):


**Figure 11** D for the M Parameter


101


ZPL Commands


(0 at the end of field data is a bogus character that is inserted as a placeholder for the check digit the
printer will automatically insert.


- Subset C using FNC1 values are automatically selected.


- Parentheses and spaces can be in the field data. '00' application identifier, followed by 17 characters,
followed by a bogus check digit placeholder.


- Check digit is automatically inserted. The printer will automatically calculate the check digit and put it
into the barcode and interpretation line.


- The interpretation line will also show the parentheses and spaces but will strip them out from the actual
barcode.


**Printing the Interpretation Line**


This example shows printing the interpretation in a different font with firmware x.11.x or later:


**Figure 12** Interpretation Line


The font command ( `^A0N,40,30` ) can be added and changed to alter the font and size of the
interpretation line.


**With firmware version later than x.10.x**


- A separate text field needs to be written.


- The interpretation line needs to be turned off.

- `^A0N,50,40` is the font and size selection for the separate text field.

- You have to make sure you enter the correct check digit in the text field.


- Creating a separate text field allows you to format the interpretation line with parentheses and spaces.


**Figure 13** Firmware Older Than X.10.X


102


ZPL Commands


**Application Identifiers — UCC/EAN APPLICATION IDENTIFIER**


An Application Identifier is a prefix code used to identify the meaning and the format of the data that
follows it (data field).


There are AIs for identification, traceability, dates, quantity, measurements, locations, and many other types
of information.


For example, the AI for batch number is 10, and the batch number AI is always followed by an alphanumeric
batch code not to exceed 20 characters.


The UCC/EAN Application Identifiers provide an open standard that can be used and understood by all
companies in the trading chain, regardless of the company that originally issued the codes.


**NOTE:** Table 4  UCC/EAN Application Identifier on page 103 is a partial table showing the
application identifiers. For more current and complete information, search the Internet for **UCC**
**Application Identifier** .


**Table 4** UCC/EAN Application Identifier



|Data Content|AI|Plus The Following<br>Data Structure|
|---|---|---|
|Serial Shipping Container Code (SSCC)|00|exactly 18 digits|
|Shipping Container Code|01|exactly 14 digits|
|Batch Numbers|10|up to 20 alphanumerics|
|Production Date (YYMMDD)|11|exactly 6 digits|
|Packaging Date (YYMMDD)|13|exactly 6 digits|
|Sell By Date (YYMMDD)|15|exactly 6 digits|
|Expiration Date (YYMMDD)|17|exactly 6 digits|
|Product Variant|20|exactly 2 digits|
|Serial Number|21|up to 20 alphanumerics|
|HIBCC Quantity, Date, Batch and Link|22|up to 29 alphanumerics|
|Lot Number|23|up to 19 alphanumerics|
|Quantity Each|30||
|Net Weight (Kilograms)|310|exactly 6 digits|
|Length, Meters|311|exactly 6 digits|
|Width or Diameter (Meters)|312|exactly 6 digits|
|Depths (Meters)|313|exactly 6 digits|
|Area (Sq. Meters)|314|exactly 6 digits|
|Volume (Liters)|315|exactly 6 digits|
|Volume (Cubic Meters)|316|exactly 6 digits|
|Net Weight (Pounds)|320|exactly 6 digits|
|Customer PO Number|400|up to 29 alphanumerics|
|Ship To (Deliver To) Location Code using EAN 13 or DUNS<br>Number with leading zeros|410|exactly 13 digits|


103




ZPL Commands


**Table 4** UCC/EAN Application Identifier (Continued)













|Data Content|AI|Plus The Following<br>Data Structure|
|---|---|---|
|Bill To (Invoice To) Location Code using EAN 13 or DUNS<br>Number with leading zeros|411|exactly 13 digits|
|Purchase from|412|exactly 13 digits|
|Ship To (Deliver To) Postal Code within single postal<br>authority|420|up to 9 alphanumerics|
|Ship To (Deliver To) Postal Code with 3-digit ISO Country<br>Code Prefix|421|3 digits plus up to 9<br>alphanumerics|
|Roll Products - width, length, core diameter, direction and<br>splices|8001|exactly 14 digits|
|Electronic Serial number for cellular mobile phone|8002|up to 20 alphanumerics|
|Plus one digit for length indication.<br>Plus one digit for decimal point indication.|Plus one digit for length indication.<br>Plus one digit for decimal point indication.|Plus one digit for length indication.<br>Plus one digit for decimal point indication.|


For date fields that only need to indicate a year and month, the day field is set to `00` .


**Chaining several application identifiers (firmware x.11.x or later)**

The FNC1, which is invoked by `>8`, is inserted just before the AIs so that the scanners reading the code see
the FNC1 and know that an AI follows.

**Example:** This is an example with the mode parameter set to `A` (automatic):

```
^XA
^BY2,2.5,193
^FO33,400
^BCN,,N,N,N,A
^FD>;>80204017773003486100008535>8910001>837252^FS
^FT33,625^AEN,0,0^FD(02)04017773003486(10)0008535(91)0001(37)252^FS
^XZ

```

**Example:** This is an example with the mode parameter set to `U` :

```
^XA
^BY3,2.5,193
^FO33,200
^BCN,,N,N,N,U
^FD>;>80204017773003486>8100008535>8910001>837252^FS
^FT33,455^A0N,30,30^FD(02)04017773003486(10)0008535(91)0001(37)252^FS
^XZ

```

**Example:** This is an example with the mode parameter set to D*:

```
^XA
^PON
^LH0,0

```

104


ZPL Commands

```
^BY2,2.5,145
^FO218,343
^BCB,,Y,N,N,D
^FD(91)0005886>8(10)0000410549>8(99)05^FS
^XZ

```

105