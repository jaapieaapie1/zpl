# ^GC




ZPL Commands


The `^GC` command produces a circle on the printed label. The command parameters specify the diameter
(width) of the circle, outline thickness, and color. Thickness extends inward from the outline.


**Graphic Circle**

**Format:** `^GCd,t,` c







|Parameters|Details|
|---|---|
|`d =` circle diameter (in<br>dots)|**Values:** `3` to`4095` (larger values are replaced with 4095)<br>**Default:** `3`|
|`t =` border thickness (in<br>dots)|**Values:** `2` to`4095`<br>**Default:** `1`|
|`c =` line color|**Values:**<br>`B =` black<br>`W =` white<br>**Default:** `B`|


**Example**


This is an example of how to create a circle on the printed label:


212