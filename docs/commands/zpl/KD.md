# ^KD




ZPL Commands


The `^KD` command selects the format that the Real-Time Clock’s date and time information presents as
on a configuration label. This is also displayed on the Printer Idle LCD control panel display, and displayed
while setting the date and time.


**Select Date and Time Format (for Real Time Clock)**

**Format:** `^KDa`






|Parameters|Details|
|---|---|
|`a =` value of date and<br>time format|**Values:**<br>`0 =` normal, displays**Version Number** of firmware<br>`1 =` MM/DD/YY (24-hour clock)<br>`2 =` MM/DD/YY (12-hour clock)<br>`3 =` DD/MM/YY (24-hour clock)<br>`4 =` DD/MM/YY (12-hour clock)<br>**Default:** `0`|



**Comments:** If the Real-Time Clock hardware is not present, Display Mode is set to 0 (Version Number).


If Display Mode is set to 0 (Version Number) and the Real-Time Clock hardware is present, the date and
time format on the configuration label is presented in format 1.


If Display Mode is set to 0 (Version Number) and the Real-Time Clock hardware is present, the date and
time format on the control panel display is presented in format 1.


For more details on select date and time format for the Real Time Clock, see Real Time Clock on page
1595.


283