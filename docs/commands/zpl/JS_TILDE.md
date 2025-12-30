# ~JS




ZPL Commands


The `~JS` command is used to control the backfeed sequence. This command can be used on printers with
or without built-in cutters. This command is ignored on the HC100 printer.


**Change Backfeed Sequence**


These are the primary applications:


- to allow programming of the rest point of the cut edge of continuous media.


- provide immediate backfeed after peel-off when the printer is used in a print/apply application
configuration.

This command stays in effect only until the printer is turned off, a new `~JS` command is sent, or the setting
is changed on the control panel. When a `~JS` command is encountered, it overrides the current control
panel setting for the Backfeed Sequence.


The most common way of eliminating backfeed is to operate in Rewind Mode. Rewind Mode does not
backfeed at all. After a label prints, the leading edge of the next label is placed at the print line. This
eliminates the need to backfeed and does not introduce a non printable area at the leading edge or
bottom of the label. It also does not allow the label to be taken from the printer because it is not fed out
from under the printhead.


Running in another mode with backfeed turned off allows the label to be removed and eliminates the timereduction of the backfeed sequence.


**Format:** ~JSb






|Parameters|Values|
|---|---|
|`b =` backfeed order in<br>relation to printing|`A` —100 percent backfeed after printing and cutting<br>`B` —0 percent backfeed after printing and cutting, and 100 percent before<br>printing the next label<br>`N` —normal — 90 percent backfeed after label is printed<br>`O` —off — turn backfeed off completely<br>`10` to`90`  —percentage value<br>The value entered must be a multiple of 10. Values not divisible by 10 are<br>rounded to the nearest acceptable value. For example,`~JS55` is accepted<br>as 60 percent backfeed.<br>**Default:** `N`|



**Comments:** When using a specific value, the difference between the value entered and 100 percent is
calculated before the next label is printed. For example, a value of 40 means 40 percent of the backfeed
takes place after the label is cut or removed. The remaining 60 percent takes place before the next label is
printed.


The value for this command is also reflected in the Backfeed parameter on the printer configuration label.

For `~JSN` - the Backfeed parameter is listed as DEFAULT

For `~JSA` - or 100% the Backfeed parameter is listed as AFTER

For `~JSB` - or 0% the Backfeed parameter is listed as BEFORE

For `~JS10` - 10% of the backfeed takes place after the label is cut or removed. The remaining 90% takes
place before the next label is printed.


276