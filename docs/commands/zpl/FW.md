# ^FW




ZPL Commands


The `^FW` command sets the default orientation for all command fields that have an orientation (rotation)
parameter (and in x.14 sets the default justification for all commands with a justification parameter). Fields
can be rotated 0, 90, 180, or 270 degrees clockwise by using this command. In x.14, justification can be left,
right, or auto.


**Field Orientation**

The `^FW` command affects only fields that follow it. Once you have issued a `^FW` command, the setting is
retained until you turn off the printer or send a new `^FW` command to the printer.

**Format:** `^FWr,z`






|Parameters|Details|
|---|---|
|`r =` rotate field|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees<br>`I =` inverted 180 degrees<br>`B =` bottom-up 270 degrees, read from the bottom up<br>**Initial Value at Power Up:** `N`|
|`z =` justification<br>The`z` parameter is<br>available only with<br>printers with firmware<br>version V60.14.x,<br>V50.14.x, or later.|**Values:**<br>`0 =` left justification<br>`1 =` right justification<br>`2 =` auto justification (script dependent)<br>**Default:** auto for`^TB` and left for all other commands|



**Example:** This example shows how `^FW` rotation works in conjunction with `^FO` . In this example, note that:

- the fields using A0N print the field in normal rotation

- the fields with no rotation indicated (A0) follow the rotation used in the `^FW` command ( `^FWR` ).


**Comments:** `^FW` affects only the orientation in commands where the rotation parameter has not been
specifically set. If a command has a specific rotation parameter, that value is used.


`^FW` affects only the justification in commands where the parameter has not been set. If a command has a
specific justification parameter, that value is used.


208