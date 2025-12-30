# ^BY




ZPL Commands


The `^BY` command is used to change the default values for the module width (in dots), the wide bar to
narrow bar width ratio and the bar code height (in dots). It can be used as often as necessary within a label
format.


**Bar Code Field Default**

**Format:** `^BYw,r,h`






|Parameters|Details|
|---|---|
|`w =` module width (in<br>dots)|**Values:** `1` to`10`<br>**Initial Value at Power Up:** `2`|
|`r =` wide bar to narrow<br>bar width ratio|**Values:** `2.0` to`3.0`, in 0.1 increments<br>This parameter has no effect on fixed-ratio bar codes.<br>**Default:** `3.0`|
|`h =` bar code height (in<br>dots)|**Initial Value at Power Up:** 10|



For parameter `r`, the actual ratio generated is a function of the number of dots in parameter `w`, module
width. See the table below. Module width and height ( `w` and `h` ) can be changed at anytime with the `^BY`
command, regardless of the symbology selected.


**Table 6** Module Width Ratios in Dots







|Ratio<br>Selected<br>(r)|Module Width in Dots (w)|Col3|Col4|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
||**1**|**2**|**3**|**4**|**5**|**6**|**7**|**8**|**9**|**10**|
|**2.0**|2:1|2:1|2:1|2:1|2:1|2:1|2:1|2:1|2:1|2:1|
|**2.1**|2:1|2:1|2:1|2:1|2:1|2:1|2:1|2:1|2:1|2.1:1|
|**2.2**|2:1|2:1|2:1|2:1|2.2:1|2.16:1|2.1:1|2.12:1|2.11:1|2.2:1|
|**2.3**|2:1|2:1|2.3:1|2.25:1|2.2:1|2.16:1|2.28:1|2.25:1|2.2:1|2.3:1|
|**2.4**|2:1|2:1|2.3:1|2.25:1|2.4:1|2.3:1|2.28:1|2.37:1|2.3:1|2.41:1|
|**2.5**|2:1|2.5:1|2.3:1|2.5:1|2.4:1|2.5:1|2.4:1|2.5:1|2.4:1|2.5:1|
|**2.6**|2:1|2.5:1|2.3:1|2.5:1|2.6:1|2.5:1|2.57:1|2.5:1|2.5:1|2.6:1|
|**2.7**|2:1|2.5:1|2.6:1|2.5:1|2.6:1|2.6:1|2.57:1|2.65:1|2.6:1|2.7:1|
|**2.8**|2:1|2.5:1|2.6:1|2.75:1|2.8:1|2.6:1|2.7:1|2.75:1|2.7:1|2.8:1|
|**2.9**|2:1|2.5:1|2.6:1|2.75:1|2.8:1|2.8:1|2.85:1|2.87:1|2.8:1|2.9:1|
|**3.0**|3:1|3:1|3:1|3:1|3:1|3:1|3:1|3:1|3:1|3:1|


**Example:** Set module width ( `w` ) to 9 and the ratio ( `r` ) to 2.4. The width of the narrow bar is 9 dots wide and
the wide bar is 9 by 2.4, or 21.6 dots. However, since the printer rounds out to the nearest dot, the wide bar
is actually printed at 22 dots. This produces a bar code with a ratio of 2.44 (22 divided by 9). This ratio is as
close to 2.4 as possible, since only full dots are printed.


148


ZPL Commands


**Comments:** Once a `^BY` command is entered into a label format, it stays in effect until another `^BY`
command is encountered.


149