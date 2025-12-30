# Sending WML Content to the Printer via the CISDFCRC16 Command


```

Use these step to send WML content to the printer via the CISDFCRC16 command.


WML files – and any .nrd files used by a WML menu structure – must be stored in the printers E: memory
location. While the files are first being transmitted to the printer, they should not be processed by the
printers ZPL formatting engine. This can be done by using the CISDFCRC16 command. This command
allows content to be written directly to the E: memory location, without being processed by the printers ZPL
formatting engine. By using the CISDFCRC16 command, WML content can be transmitted to the printer via
the Serial, USB or Parallel ports.


For additional information on the CISDFCRC16 command, see .

**1.** To send the sample `index.wml` shown earlier, send the following commands to the printer:

```
         ! CISDFCRC16
         0000
         INDEX.WML
         0000004E
         0000
         <wml>
         <display>
         <card>
         <p>Hello World!!</p>
         </card>
         </display>
         </wml>

```

1703


Wireless Markup Language (WML)


**2.** Power cycle the printer.


Once the printer completes the power cycle the display should look similar to this:


For additional index.wml examples, see WML Examples .


**NOTE:** When a WML menu is resident on the printer, the standard menu system can be easily
be accessed by holding down the Cancel and Setup/Exit buttons (on the ZM400) or Cancel
and Setup/Exit buttons (on Xi4) on the front panel while the printer powers up. Hold the
buttons down until the PRINT READY message displays on the front panel. To return to the
WML defined menu, reset the printer again.