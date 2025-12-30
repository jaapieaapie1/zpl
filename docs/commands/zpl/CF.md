# ^CF




ZPL Commands


The `^CF` command sets the default font used in your printer. You can use the `^CF` command to simplify
your programs.


**Change the Alphanumeric Default Font**

**Format:** `^CFf,h,w`






|Parameters|Details|
|---|---|
|`f =` specified default font|**Values:** `A` through`Z` and`0` to`9`<br>**Initial Value at Power Up:**`A`|
|`h =` individual character<br>height (in dots)|**Values:** `0` to`32000`<br>**Initial Value at Power Up:**`9`|
|`w =` individual character<br>width (in dots)|**Values:** `0` to`32000`<br>**Initial Value at Power Up:**`5` or last permanent saved value|



Parameter `f` specifies the default font for every alphanumeric field. Parameter `h` is the default height for
every alphanumeric field, and parameter w is the default width value for every alphanumeric field.


The default alphanumeric font is A. If you do not change the alphanumeric default font and do not use any
alphanumeric field command ( `^AF` ) or enter an invalid font value; any data you specify prints in font A.

Defining only the height or width forces the magnification to be proportional to the parameter defined. If
neither value is defined, the last `^CF` values given or the default `^CF` values for height and width are used.


**Example**

This is an example of `^CF` code and the result of the code:


**Comments:** Any font in the printer, including downloaded fonts, EPROM stored fonts, and fonts A through
Z and 0 to 9, can also be selected with `^CW` .


154