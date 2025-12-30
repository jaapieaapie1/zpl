# ^XB




ZPL Commands


The `^XB` command suppresses forward feed of media to tear-off position depending on the current printer
mode. Because no forward feed occurs, a backfeed before printing of the next label is not necessary; this
improves throughput. When printing a batch of labels, the last label should not contain this command.


**Suppress Backfeed**

**Format:** `^XB`


**^XB in Tear-off Mode**


Normal Operation: backfeed, print, and feed to rest

`^XB` Operation: print (Rewind Mode)


**^XB in Peel-off Mode**


Normal Operation: backfeed, print, and feed to rest

`^XB` Operation: print (Rewind Mode)


**NOTE:** To prevent jamming in cutter mode, `^XB` suppresses backfeed and cutting.


370




## **^XF**



ZPL Commands


The `^XF` command recalls a stored format to be merged with variable data. There can be multiple `^XF`
commands in one format, and they can be located anywhere within the code.


**Recall Format**

When recalling a stored format and merging data using the `^FN` (Field Number) function, the calling format
must contain the `^FN` command to merge the data properly.

While using stored formats reduces transmission time, no formatting time is saved. The ZPL II format being
recalled is saved as text strings that need to be formatted at print time.

**Format:** `^XFd:o.x`







|Parameters|Details|
|---|---|
|`d =` source device of<br>stored image|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** search priority (`R:`, `E:`, `B:`, and`A:`)|
|`o =` name of stored<br>image|**Values:** 1 to 8 alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension l|**Fixed Value:** `.ZPL`|


For a complete example of the `^DF` and `^XF` command, seeExercise 6: ^DF and ^XF - Download Format
and Recall Format.


371


## **^XG**



ZPL Commands


The `^XG` command is used to recall one or more graphic images for printing. This command is used in a
label format to merge graphics, such as company logos and piece parts, with text data to form a complete
label.


**Recall Graphic**


An image can be recalled and resized as many times as needed in each format. Other images and data
might be added to the format.

**Format:** `^XGd:o.x,mx,my`












|Parameters|Details|
|---|---|
|`d =` source device of<br>stored image|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** search priority (`R:`, `E:`, `B:`, and`A:`)|
|`o =` name of stored<br>image|**Values:** `1` to`8` alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension l|**Fixed Value:** `.GRF`|
|`mx =` magnification factor<br>on the x-axis|**Values:** `1` to`10`<br>**Default:**`1`|
|`my =` magnification factor<br>on the y-axis|**Values:** `1` to`10`<br>**Default:**`1`|



**Example:** This is an example of using the `^XG` command to recall the image `SAMPLE.GRF` from DRAM and
print it in five different sizes in five different locations on the same label:

```
^XA
^FO100,100^XGR:SAMPLE.GRF,1,1^FS
^FO100,200^XGR:SAMPLE.GRF,2,2^FS
^FO100,300^XGR:SAMPLE.GRF,3,3^FS
^FO100,400^XGR:SAMPLE.GRF,4,4^FS
^FO100,500^XGR:SAMPLE.GRF,5,5^FS
^XZ

```

372


## **^XS**



ZPL Commands


The `^XS` command controls whether dynamic media calibration is performed to compensate for variations
in label length, position, transmissivity, and/or reflectance after a printer is powered-up or the printer has
been opened (for example to change or check the media).


**Set Dynamic Media Calibration**

**Format:** `^XSlength,threshold`






|Parameters|Details|
|---|---|
|`length =` dynamic<br>length calibration|**Values:**<br>`Y =` enable<br>`N =` disable<br>**Default:** `Y`|
|`threshold =`<br>dynamic threshold<br>calibration|**Values:**<br>`Y =` enable<br>`N =` disable<br>**Default:** `Y`|
|`gain =` dynamic gain<br>calibration (to be in a<br>future implementation)|**Values:**<br>`Y =` enable<br>`N =` disable<br>**Default:** `Y`|



373


## **^XZ**



ZPL Commands


The `^XZ` command is the ending (closing) bracket. It indicates the end of a label format. When this
command is received, a label prints. This command can also be issued as a single ASCII control character
ETX (Control-C, hexadecimal 03).


**End Format**

**Format:** `^XZ`

**Comments:** Label formats must start with the `^XA` command and end with the `^XZ` command to be in valid
ZPL II format.


374


## **^ZZ**



ZPL Commands


The `^ZZ` command places the printer in an idle or shutdown mode.


**Printer Sleep**

**Format:** `^ZZt,b`






|Parameters|Details|
|---|---|
|`t =` number of second<br>(idle time) prior to<br>shutdown|**Values:** `0` to`999999` – setting`0` disables automatic shutdown<br>**Default:** last permanently saved value or`0`|
|`b =` label status at<br>shutdown|**Values:**<br>`Y =` indicates to shutdown when labels are still queued<br>`N =` indicates all labels must be printed before shutting down<br>**Default:** `N`|



**Comments:** The `^ZZ` command is only valid on the PA400 and PT400 battery-powered printers.


375