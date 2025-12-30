# ^FO




ZPL Commands


The `^FO` command sets a field origin, relative to the label home ( `^LH` ) position. `^FO` sets the upper-left
corner of the field area by defining points along the x-axis and y-axis independent of the rotation.


**Field Origin**

**Format:** `^FOx,y,z`






|Parameters|Details|
|---|---|
|`x =` x-axis location (in<br>dots)|**Values:** `0` to`32000`<br>**Default:** `0`|
|`y =` y-axis location (in<br>dots)|**Values:** `0` to`32000`<br>**Default:** `0`|
|`z =` justification<br>The`z` parameter is only<br>supported in firmware<br>versions V60.14.x,<br>V50.14.x, or later.|**Values:**<br>`0 =` left justification<br>`1 =` right justification<br>`2 =` auto justification (script dependent)<br>**Default:** last accepted`^FW` value or`^FW` default|



**Comments:** If the value entered for the `x` or `y` parameter is too high, it could position the field origin
completely off the label.


This command interacts with the field direction parameter of `^FP` and with the rotation parameter of `^A` .
For output and examples, see Field Interactions on page 1588.


The auto justification option might cause unexpected results if variable fields or bidirectional text are
used with `^FO` . For the best results with bidirectional text and/or variable fields, use either the left of right
justification option.


201