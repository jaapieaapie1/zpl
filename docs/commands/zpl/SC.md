# ^SC




ZPL Commands


The `^SC` command allows you to change the serial communications parameters you are using.


**Set Serial Communications**

**Format:** `^SCa,b,c,d,e,f`







|Parameters|Details|
|---|---|
|`a =` baud rate|**Values:** `110`1; `300`; `600`; `1200`; `2400`; `4800`; `9600`; `14400`; `19200`; `28800`;<br>`38400`; or`57600;` `115200`<br>**Default:** must be specified or the parameter is ignored|
|`b =` word length (in data<br>bits)|**Values:** `7` or`8`<br>**Default:** must be specified|
|`c =` parity|**Values:** `N` (none),`E` (even), or`O` (odd)<br>**Default:** must be specified|
|`d =` stop bits|**Values:** `1` or`2`<br>**Default:** must be specified|
|`e =` protocol mode|**Values:**<br>`X =` XON/XOFF<br>`D =` DTR/DSR<br>`R =` RTS<br>`M =` DTR/DSR XON/XOFF2<br>**Default:** must be specified|
|`f =` Zebra protocol|**Values:**<br>`A =` ACK/NAK<br>`N =` none<br>`Z =` Zebra<br>**Default:** must be specified|


1. This value is not supported on Xi4, RXi4, ZM400/ZM600, RZ400/RZ600, and S4M printers.


2. This parameter is supported only on G-Series printers. Using the DTR/DSR XON/XOFF mode will cause
the printer to respond to either DTR/DSR or XON/XOFF, depending on which method is first received from
the host device.


**Comments:** If any of the parameters are missing, out of specification, not supported by a particular printer,
or have a ZPL-override DIP switch set, the command is ignored.

A `^JUS` command causes the changes in Communications Mode to persist through power-up and software
resets.


331