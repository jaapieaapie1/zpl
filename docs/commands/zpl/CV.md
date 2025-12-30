# ^CV




ZPL Commands


The `^CV` command acts as a switch to turn the code validation function on and off. When this command is
turned on, all bar code data is checked for these error conditions:


**Code Validation**


- character not in character set


- check-digit incorrect


- data field too long (too many characters)


- data field too short (too few characters)


- parameter string contains incorrect data or missing parameter


When invalid data is detected, an error message and code is printed in reverse image in place of the bar
code. The message reads `INVALID - X` where `X` is one of these error codes:

`C =` character not in character set

`E =` check-digit incorrect

`L =` data field too long

`S =` data field too short

`P =` parameter string contains incorrect data

(occurs only on select bar codes)

Once turned on, the `^CV` command remains active from format to format until turned off by another `^CV`
command or the printer is turned off. The command is not permanently saved.

**Format:** `^CVa`

|Parameters|Details|
|---|---|
|`a =` code validation|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|



**Example:** The examples below show the error labels `^CVY` generates when incorrect field data is entered.
Compare the letter following **INVALID –** to the listing on the previous page.


**Comments:** If more than one error exists, the first error detected is the one displayed.

The `^CV` command tests the integrity of the data encoded into the bar code. It is not used for (or to be
confused with) testing the scan-integrity of an image or bar code.


167