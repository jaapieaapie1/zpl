# ^GB




ZPL Commands


The `^GB` command is used to draw boxes and lines as part of a label format. Boxes and lines are used to
highlight important information, divide labels into distinct areas, or improve the appearance of a label. The
same format command is used for drawing either boxes or lines.


**Graphic Box**

**Format:** `^GBw,h,t,c,r`












|Parameters|Details|
|---|---|
|`w =` box width (in dots)|**Values:** value of`t` to`32000`<br>**Default:** value used for thickness (t) or 1|
|`h =` box height (in dots)|**Values:** value of`t` to`32000`<br>**Default:** value used for thickness (t) or 1|
|`t =` border thickness (in<br>dots)|**Values:** `1` to`32000`<br>**Default:** `1`|
|`c =` line color|**Values:**<br>`B =` black<br>`W =` white<br>**Default:** `B`|
|`r =` degree of corner-<br>rounding|**Values:** `0` (no rounding) to`8` (heaviest rounding)<br>**Default:** `0`|



For the `w` and `h` parameters, keep in mind that printers have a default of 6, 8, 12, or 24 dots/millimeter.
This comes out to 153, 203, 300, or 600 dots per inch. To determine the values for w and h, calculate the
dimensions in millimeters and multiply by 6, 8, 12, or 24.


If the width and height are not specified, you get a solid box with its width and height as specified by the
value `t` .

The roundness index is used to determine a rounding radius for each box. Formula:


rounding-radius = (rounding-index / 8) * (shorter side / 2)


where the shorter side is the lesser of the width and height (after adjusting for minimum and default
values).


**Example:** Here are a few examples of graphic boxes:


**Width: 1.5 inch; Height: 1 inch; Thickness: 10; Color: default; Rounding: default**


**Width: 0 inch; Height: 1 inch; Thickness: 20; Color: default; Rounding: default:**


210




ZPL Commands


**Width: 1 inch; Height: 0 inch; Thickness: 30; Color: default; Rounding: default**


**Width: 1.5 inch; Height: 1 inch; Thickness: 10; Color: default; Rounding: 5**


211