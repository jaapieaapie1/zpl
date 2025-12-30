# Send WML Content to the Printer via FTP



Use these steps to send WML content to the printer via FTP.


Go to a command prompt.

At the command line prompt, type `"ftp xxx.xxx.xxx.xxx"`, where `xxx.xxx.xxx.xxx` is the IP
Address of the printer. For example, if the IP Address of the printer is 10.3.5.34, the command would be:

```
       ftp 10.3.5.34

```

**1.** Press Enter to connect to the printer.


**2.** Press Enter to log in to the printer.


**3.** At the FTP prompt, type "put index.wml" and press Enter. The index.wml file will be transferred to the
printer’s E: memory.


**4.** Type "quit" to disconnect from the printer and exit FTP.


**5.** Power cycle the printer.


Once the printer completes the power cycle the display should look similar to this:


For additional index.wml examples, see WML Examples.


**NOTE:** When a WML menu is resident on the printer, the standard menu system can be easily
be accessed by holding down the Cancel and Setup/Exit buttons (on the ZM400) or the
Cancel and Setup/Exit buttons (on Xi4) or the Select button (on GX) on the front panel while
the printer powers up. Hold the buttons down until the PRINT READY message displays on
the front panel. To return to the WML defined menu, reset the printer again.


**IMPORTANT:** When using the `“ip.ftp.execute_file”` command, be sure to reset the
command back to `“on”` for use in production processes. If the setting is left in the `“off”`
configuration, when label formats or firmware are sent to the printer via FTP they will not be
processed as intended – and the E: memory location can quickly become full.


1702


Wireless Markup Language (WML)