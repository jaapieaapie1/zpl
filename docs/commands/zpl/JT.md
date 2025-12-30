# ^JT




ZPL Commands


The `^JT` command allows you to change the printhead test interval from every 100 labels to any desired
interval. With the `^JT` command, the printer is allowed to run the test after printing a label. When a
parameter is defined, the printer runs the test after printing a set amount of labels.


**Head Test Interval**


The printer’s default head test state is off. Parameters for running the printhead test are defined by the
user.

**Format:** `^JT####,a,b,c`






|Parameters|Details|
|---|---|
|`#### =` four-digit number<br>of labels printed between<br>head tests|**Values:** `0000` to`9999`<br>If a value greater than`9999` is entered, it is ignored.<br>**Default:**` 0000` (off)|
|`a =` manually select<br>range of elements to test|**Values:**<br>`N =` no<br>`Y =` yes<br>**Initial Value at Power Up:** `N`|
|`b =` first element to check<br>when parameter`a` is Y|**Values:** `0` to`9999`<br>**Initial Value at Power Up:** `0`|
|`c =` last element to check<br>when parameter`a` is Y|**Values:** `0` to`9999`<br>**Initial Value at Power Up:** `9999`|



**Comments:** The `^JT` command supports testing a range of print elements. The printer automatically
selects the test range by tracking which elements have been used since the previous test.

`^JT` also turns on Automatic Mode to specify the first and last elements for the head test. This makes it
possible to select any specific area of the label or the entire print width.


If the last element selected is greater than the print width selected, the test stops at the selected print
width.


Whenever the head test command is received, a head test is performed on the next label unless the count
is set to 0 (zero).


277