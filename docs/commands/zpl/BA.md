# ^BA




ZPL Commands


The `^BA` command creates a variable length, continuous symbology. The Code 93 barcode is used in
many of the same applications as Code 39. It uses the full 128-character ASCII set. ZPL II, however, does
not support ASCII control codes or escape sequences. It uses the substitute characters shown below.


**Code 93 Barcode**

|Control Code|ZPL II Substitute|
|---|---|
|Ctrl $|&|
|Ctrl %|‘|
|Ctrl /|(|
|Ctrl +|)|



Each character in the Code 93 barcode is composed of six elements: three bars and three spaces.
Although invoked differently, the human-readable interpretation line prints as though the control code has
been used.

- `^BA` supports a fixed print ratio.

- Field data ( `^FD` ) is limited to the width (or length, if rotated) of the label.


**IMPORTANT:** If additional information about this bar code is required, refer to
[www.aimglobal.org.](http://www.almglobal.com)

**Format:** `^BAo,h,f,g,e`





|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` bar code height (in<br>dots)|**Values:** `1` to`32000`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|


87


ZPL Commands

|Parameters|Details|
|---|---|
|`e =` print check digit|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|



**Example**


This is an example of a Code 93 barcode:


**Comments:** All control codes are used in pairs. Code 93 is also capable of encoding the full 128-character
ASCII set.


**Full ASCII Mode for Code 93**


Code 93 can generate the full 128-character ASCII set using paired characters as shown in the following
tables.


88


# ASCII Code 93



ZPL Commands

# ASCII Code 93



Space



NUL
SOH

STX
ETX
EOT
ENQ

ACK

BEL

BS
HT

LF
VT
FF
CR
SO

SI
DLE
DC1
DC2
DC3
DC4
NAK
SYN

ETB
CAN

EM
SUB
ESC

FS
FS
RS
US



‘U
&A

&B
&C
&D
&E
&F
&G
&H

&I
&J
&K

&L
&M

&N
&O

&P
&Q
&R
&S
&T
&U
&V
&W

&X
&Y
&Z

‘A
‘B
‘C
‘D
‘E



SP

!
“
#
$
%



,

.
/
0
1
2
3
4
5
6
7
8
9

:
;
<
=

?



(A
(B



&



(
(
(
(
(
(



(C
(D
(E
(F

G

(H



‘
(
)

 ++



(I
(J

++



(L

 .
/
O

1
2
3
4
5
6
7
8
9
(Z

‘F
‘G
‘H

‘I
‘J



(
(



89