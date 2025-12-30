# ^GD




ZPL Commands


The `^GD` command produces a straight diagonal line on a label. This can be used in conjunction with other
graphic commands to create a more complex figure.


**Graphic Diagonal Line**

**Format:** `^GDw,h,t,c,o`









|Parameters|Details|
|---|---|
|`w =` box width (in dots)|**Values:** `3` to`32000`<br>**Default:** value of`t` (thickness) or`3`|
|`h =` box height (in dots)|**Values:** `3` to`32000`<br>**Default:** value of`t` (thickness) or`3`|
|`t =` border thickness (in<br>dots)|**Values:** 1 to 32000<br>**Default:** 1|
|`c =` line color|**Values:**<br>`B =` black<br>`W =` white<br>**Default:** `B`|
|`o =` orientation (direction<br>of the diagonal)|**Values:**<br>R (or /)`=` right-leaning diagonal (or \)`=` left-leaning diagonal**Default:** `R`|


**Example**





This is an example of how to create a diagonal line connecting one corner with the opposite corner of a
box on a printed label:


213