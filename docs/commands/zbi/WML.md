# WML Examples


```

The examples shown below “build” from a simple, display-only, WML menu to a more complex interactive
example that uses .nrd files containing ZPL commands. In the initial examples, all lines are explained in
detail, in the later examples only the new concepts are covered in detail.


Indenting is used in the examples below to improve readability, it is not necessary in actual use.

## **Example 1**


This example shows a basic WML menu structure that uses only fixed text. The content below shows the
WML script plus numbered callouts and a table that identify the function of each of the WML tags.



1 →


2 →


3 →


4 →


5 →


6 →


7 →


```
<wml>
<display>
<card>
<p>Hello World!!</p>
</card>
</display>
</wml>

```


|1|Beginning of the WML file.|
|---|---|
|2|Beginning of the content to be displayed.|
|3|The`<card>` tag begins the definition of this menu.|
|4|The`<p>` beings a paragraph, here displaying Hello World! The`</p>` ends the paragraph.|
|5|The`</card>` tag ends the definition of this menu.|
|6|End of the content to be displayed.|
|7|End of the WML file.|


In use, this WML menu looks similar to this:


1705


Wireless Markup Language (WML)

## **Example 2**


This example demonstrates a WML menu structure that uses fixed text, plus two SGD commands to display
the current printer settings for the Baud rate and ESSID settings on the printer.



1 →


2 →


3 →


4 →


5 →


6 →


7 →


8 →


9 →


```
<wml>
<display>
<card>
<p>Baud: $(comm.baud)</p>
<br/>
<p>AP: $(wlan.essid)</p>
</card>
</display>
</wml>

```


|1|Beginning of the WML file.|
|---|---|
|2|Beginning of the content to be displayed.|
|3|The`<card>` tag begins the definition of this menu.|
|4|The`<p>` begins a paragraph.‘Baud:’ displays the text Baud:$(comm.baud) retrieves and<br>displays the printers’ current baud rate. The`</p>` ends the paragraph.|
|5|A line break|
|6|The`<p>` begins a paragraph.“AP:” displays the text AP:$(wlan.essid) retrieves and displays the<br>printers’ current ESSID setting. The`</p>` ends the paragraph.|
|7|The`</card>` tag ends the definition of this menu.|
|8|End of the content to be displayed.|
|9|End of the WML file|


In use, this WML menu looks similar to this:


1706


Wireless Markup Language (WML)

## **Example 3**


This example demonstrates a WML menu structure with two menus.


Fixed text and SGD commands are used to display the current printer settings for the Baud rate and ESSID
settings on menu one and the Firmware version and ZBI State on menu two. Through use of the ‘timer’
setting, the menu will automatically return to a defined WML card if no buttons are pressed after a set time
period. The menu is configured to allow printer alerts (such as HEAD OPEN) to be displayed.



1→

2→

3→

4→

5→

6→

7→

8→

9→

10→

11→

12→

13→

14→

15→

16→

17→

18→

19→

20→

21→


```
<wml>
<display>
<card id="main" title="" ontimer="#main" alerts="on">
<timer value="50"></timer>
<p>Baud: $(comm.baud)</p>
<br/>
<p>AP: $(wlan.essid)</p>
<p> </p><br/>
<p> </p><br/>
<p> <a href="#system">Firmware</a></p>
</card>
<card id="system" title="" ontimer="#main" alerts="on">
<timer value="50"></timer>
<p>Firmware:</p><br/>
<p>$(appl.name)</p><br/>
<p>ZBI State:</p><br/>
<p>$(zbi.key)</p><br/>
<p> <a href="#main">Main</a></p>
</card>
</display>
</wml>

```

1707


Wireless Markup Language (WML)

|3|<card id="main" – defines the card’s id – "main".title="" – defines the title (not displayed<br>on screen).ontimer="#main" – defines the WML card to display when the timer runs<br>out.alerts="on"> – enables the alerts display feature.|
|---|---|
|4|<timer value="50"></timer> – sets the timer to 50 (in 10th of a second increments).|
|10|<p> <a href="#system">Firmware</a></p> – defines a link to the “system” card.|
|12|<card id="system" – defines the card’s id – "system".title="" – defines the title (not displayed<br>on screen).ontimer="#main" – defines the WML card to display when the timer runs<br>out.alerts="on"> – enables the alerts display feature.|
|13|<timer value="50"></timer> – sets the timer to 50 (in 10th of a second increments).|
|18|<p> <a href="#main">Main</a></p> – defines a link to the “main” card.|



In use, these two WML menus look similar to this:


**NOTE:** GX series printers can display four lines of text. If you are using a GX series printer,
remove one line of text from each “card” to use this example.


1708


Wireless Markup Language (WML)

## **Example 4**


This example demonstrates a WML menu structure that creates two menu screens and a link to a
command file – “config.nrd” – that contains a ZPL command that will cause the unit to print a configuration
label.



1→

2→

3→

4→

5→

6→

7→

8→

9→

10→

11→

12→

13→

14→

15→

16→

17→

18→

19→

20→

21→

22→

23→

24→

25→

26→

27→

28→

29→

30→


18

19


```
 <wml>
 <display>
 <card id="main" title="" ontimer="#main" alerts="on">
 <timer value="50"></timer>
 <p>Baud: $(comm.baud)</p>
 <br/>
 <p>AP: $(wlan.essid)</p>
 <p> </p><br/>
 <p> </p><br/>
 <p> <a href="#system">Firmware</a></p>
 </card>
 <card id="system" title="" ontimer="#main" alerts="on">
 <timer value="50"></timer>
 <p>Firmware:</p><br/>
 <p>$(appl.name)</p><br/>
 <p>ZBI State:</p><br/>
```

`<p>$(zbi.` key)</p><br/>
```
 <p><a href="#main">Main</a>
 <a href="#config">Config</a></p>
 </card>
 <card id="config" title="" ontimer="#main" alerts="on">
 <timer value="50"></timer>
 <p>Printing </p><br/>
 <p> Config Label...</p><br/>
 <p></p><br/>
 <p>Please wait...</p><br/>
 <setvar name="file.run" value="e:config.nrd"/>
 </card>
 </display>
 </wml>

<p><a href="#main">Main</a>
<a href="#config">Config</a></p>
```

- Defines two links, positioned next to each other - to the "main" and "config" WML cards


1709


Wireless Markup Language (WML)


27 `<setvar name="file.run" value="e:config.nrd"/>`

        - Defines that the SGD command `"file.run"` should be used on the `"e:config.nrd"`
file.

        - In this instance, the `"e:config.nrd"` file contains a single ZPL command - `"~wc"` .


In use, these WML menus look similar to this:


**NOTE:** GX series printers can display four lines of text. If you are using a GX series printer,
remove one line of text from each “card” to use this example.


1710




Wireless Markup Language (WML)

### **Example 5**


This example demonstrates a WML menu structure with three cards. The "darkness" card leverages WML
and the SGD "print.tone" command to allow the user to both view and configure a setting.


1711


1→

2→

3→

4→

5→

6→

7→

8→

9→

10→

11→

12→

13→

14→

15→

16→

17→

18→

19→

20→

21→

22→

23→

24→

25→

26→

27→

28→

29→

30→

31→

32→

33→

34→

35→

36→

37→



Wireless Markup Language (WML)

```
<wml>
<display>
<card id="main" title="" ontimer="#main" alerts="on">
<timer value="50"></timer>
<p>Baud: $(comm.baud)</p>
<br/>
<p>AP: $(wlan.essid)</p>
<p> </p><br/>
<p> </p><br/>
<p> <a href="#system">Firmware</a></p>
</card>
<card id="system" title="" ontimer="#main" alerts="on">
<timer value="50"></timer>
<p>Firmware:</p><br/>
<p>$(appl.name)</p><br/>
<p>ZBI State:</p><br/>
<p>$(zbi.key)</p><br/>
<p><a href="#main">Main</a> <a href="#darkness">Darkness</a></p>
</card>
<card id="darkness" title="" ontimer="#main" alerts="on">
<timer value="50"></timer>
<p>Current: $(print.tone)</p><br/>
<p>Change: </p><do type="accept" label="Up"><setvar
name="print.tone" value="+1.0"/></do><br/>
<p>Change: </p><do type="accept" label="Down"><setvar
name="print.tone" value="-1.0"/></do><br/>
<p> </p><br/>
<p><a href="#main">Main</a> <a href="#config">Config</a></p>
</card>
<card id="config" title="" ontimer="#main" alerts="on">
<timer value="50"></timer>
<p>Printing </p><br/>
<p> Config Label...</p><br/>
<p></p><br/>
<p>Please wait...</p><br/>
<setvar name="file.run" value="e:config.nrd"/>
</card>
</display>
</wml>

```

1712


Wireless Markup Language (WML)


23 `<p>Change: </p>`

      - Defines the fixed text `"Change: "`
```
    <do type="accept" label="Up"><setvar name="print.tone" value="+1.0"/
    ></do><br/>
```

      - Defines selecting the word "Up" as equal to sending the value "+1.0" for the SGD command
"print.tone". In this case, this increases the setting by 1.0.


24 `<p>Change: </p>`

      - Defines the fixed text `"Change: "`
```
    do type="accept" label="Down"><setvar name="print.tone" value="-1.0"/
    ></do><br/>
```

      - Defines selecting the word "Down" as equal to sending the value "-1.0" for the SGD
command "print.tone". In this case, this decreases the setting by 1.0.


In use, these WML menus look similar to this:


1713


Wireless Markup Language (WML)


**NOTE:** GX series printers can display four lines of text. If you are using a GX series printer,
remove one line of text from each “card” to use this example.


1714


Wireless Markup Language (WML)

## **Troubleshooting Scenarios**





|Problem Scenario|Corrective Actions|
|---|---|
|I loaded a WML menu structure<br>on the printer, but the Factory<br>menu structure is displaying.|•<br>The WML files may have syntax errors. Reconfirm that the<br>correct syntax has been used. When creating WML files it is<br>recommended to start with a simple structure, validate that it's<br>functional and build additional content onto the "known good"<br>example.<br>•<br>Power cycle the printer and watch the start-up sequence - if<br>a "WML ERROR" message displays during the start-up, the<br>`index.wml` file has a syntax error that needs to be corrected.<br>•<br>The`index.wml` file may not have been successfully transferred<br>to the printer. Use a terminal emulation program and the<br>following command to retrieve the`index.wml` file to the PC for<br>examination:<br>`! U1 setvar "file.type" "E:INDEX.WML"`<br>•<br>The`index.wml` file may not be present in the E: memory<br>location. Validate that the file is present and correctly named.<br>•<br>Confirm that straight quotes were used in all instances where the<br>quote character was used (use the " character - not " or ?). SGD<br>commands require the use of the straight quote.<br>•<br>Confirm that the "WML Menu Cancel" buttons, (Setup and Cancel<br>or Cancel and Setup/Exit or Select) were not held down during<br>start up. These actions will cause the standard menu to display.|
|Some characters in the menu<br>are cut off or some lines are not<br>displaying at all.|•<br>Characters that extend past the width of the display are truncated,<br>reposition the field as needed.<br>•<br>Check that you have not exceeded that maximum number of lines<br>the display allows (5 lines on ZM and Xi4 series units, 4 lines on<br>the GX series).|
|My WML menu structure is<br>displaying, but one of the<br>“cards” is not displaying or is<br>unreachable.|•<br>The missing "card" may not have been linked to from any of<br>the visible "cards". Review you WML content to ensure that the<br>correct links exist.<br>•<br>The WML files may have syntax errors, reconfirm that the<br>correct syntax has been used. When creating WML files it is<br>recommended to start with a simple structure, validate that<br>it’s functional and build additional content onto a known good<br>example.<br>•<br>If the missing card content is contained in a separate`.wml`<br>`file`, confirm that the necessary`.wml` files have been<br>transferred to the printer.|


1715


Wireless Markup Language (WML)





|Problem Scenario|Corrective Actions|
|---|---|
|My WML menu structure uses<br>SGD commands to display<br>current settings, but the settings<br>are not displaying.|•<br>Validate that the correct syntax was used for the SGD command.<br>•<br>Check the manual page for the command being used. Confirm<br>that the command is supported by the printer & firmware. Use a<br>terminal emulation program to send just the command being used<br>to validate it functions outside the WML menu structure.<br>•<br>Confirm that straight quotes were used in all instances where the<br>quote character was used (use the " character – not “ or ). SGD<br>commands require the use of the straight quote.<br>•<br>Characters that extend past the width of the display are truncated,<br>reposition the field as needed.<br>•<br>Check that you have not exceeded that maximum number of lines<br>the display allows (5 on ZM and Xi4 series units).|
|My WML menu structure used<br>SGD commands to allow the<br>user to alter printer settings,<br>but the settings are not getting<br>changed as expected.|•<br>Validate that the value being used in the value= parameter of the<br><do> tag is supported by the SGD command.<br>•<br>Validate that the correct syntax was used for the SGD command.<br>•<br>Check the manual page for the command being used. Confirm<br>that the command is supported by the printer & firmware. Use a<br>terminal emulation program to send just the command being used<br>to validate it functions outside the WML menu structure.<br>•<br>Confirm that straight quotes were used in all instances where the<br>quote character was used (use the " character – not “ or ). SGD<br>commands require the use of the straight quote.|
|My WML menu structure<br>uses .nrd files to send<br>commands to the printers ZPL or<br>SGD engine, but the commands<br>don’t seem to be getting sent<br>when the user selects the on-<br>screen link for the action.|•<br>Confirm that the .nrd files are present in E: memory and named as<br>expected. Resend or rename the files if necessary.<br>•<br>Confirm that the WML menu structure is using the correct file<br>name(s).<br>•<br>Confirm that the commands in the files work as expected,<br>independently of the WML menu or`.nrd` file.<br>•<br>Validate that ZPL and SGD commands have not been interlaced.|
|I am using the`CISDFCRC16`<br>command to transfer files, but<br>the files are either not being<br>transferred to the printer or are<br>showing up with a zero (0) byte<br>size.|•<br>Confirm that the Hexadecimal value used for the File Size<br>parameter is correct. This value must be an eight digit file size<br>specified in hexadecimal which indicates the number of bytes in<br>the <data> section of the command. See the full manual page on<br>the CISDFCRC16 command for additional details.<br>•<br>Validate that the CRC and Checksum parameters are correct<br>(using the "0000" value for these parameters is recommended).<br>•<br>Confirm that the WML file name and extension are in upper case<br>characters.<br>•<br>Confirm that the exclamation mark (!) was included before the<br>command name (`"! CISDFCRC16"`).|


1716