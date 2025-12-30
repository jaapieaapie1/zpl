# ^GE




ZPL Commands


The `^GE` command produces an ellipse in the label format.


**Graphic Ellipse**

**Format:** `^GEw,h,t,c`







|Parameters|Details|
|---|---|
|`w =` ellipse width (in dots)|**Values:** 3 to 4095 (larger values are replaced with 4095)<br>**Default:** the value used for thickness (`t`) or`1`|
|`h =` ellipse height (in<br>dots)|**Values:** 3 to 4095<br>**Default:** the value used for thickness (`t`) or`1`|
|`t =` border thickness (in<br>dots)|**Values:** 2 to 4095<br>**Default:** 1|
|`c =` line color|**Values:**<br>`B =` black<br>`W =` white<br>**Default:** `B`|


**Example**


This is an example of how to create an ellipse on a printed label:


214