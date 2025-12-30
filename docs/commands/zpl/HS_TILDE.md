# ~HS




ZPL Commands


When the host sends `~HS` to the printer, the printer sends three data strings back. Each string starts with an
< `STX` - control code and is terminated by an < `ETX` >< `CR` >< `LF` - control code sequence. To avoid confusion,
the host prints each string on a separate line.


**Host Status Return**


**NOTE:** When a `~HS` command is sent the printer will not send a response to the host if the printer
is in one of these conditions:


- MEDIA OUT


- RIBBON OUT


- HEAD OPEN


- REWINDER FULL


- HEAD OVER-TEMPERATURE


**String 1**

```
<STX>aaa,b,c,dddd,eee,f,g,h,iii,j,k,l<ETX><CR><LF>

```

`aaa` communication (interface) settings [1]

`b` paper out flag (1 `=` paper out)

`c` pause flag (1 `=` pause active)

`dddd` label length (value in number of dots)

`eee` number of formats in receive buffer

`f` buffer full flag (1 = receive buffer full)

`g` communications diagnostic mode flag (1 = diagnostic mode active)

`h` partial format flag (1 = partial format in progress)

`iii` unused (always 000)

`j` corrupt RAM flag (1 = configuration data lost)

`k` temperature range (1 `=` under temperature)

`l` temperature range (1 `=` over temperature)


1. This string specifies the printer’s baud rate, number of data bits, number of stop bits, parity setting, and
type of handshaking. This value is a three-digit decimal representation of an eight-bit binary number. To
evaluate this parameter, first convert the decimal number to a binary number.

```
aaa=a [8] a [7] a [6] a [5] a [4] a [3] a [2] a [1] a [0]

```

The nine-digit binary number is read according to this table:


233


|a7 = Handshake<br>• 0 = Xon/Xoff<br>• 1 = DTR|a8 a2 a1 a0 = Baud<br>0 000 = 110<br>0 001 = 300<br>0 010 = 600<br>0 011 = 1200<br>0 100 = 2400<br>0 101 = 4800<br>0 110 = 9600<br>0 111 = 19200<br>1 000 = 28800 (available only on certain printer models)<br>1 001 = 38400 (available only on certain printer models)<br>1 010 = 57600 (available only on certain printer models)<br>1 011 = 14400|
|---|---|
|a6 = Parity Odd/Even<br>•<br>0 = Odd<br>•<br>1 = Even|a6 = Parity Odd/Even<br>•<br>0 = Odd<br>•<br>1 = Even|
|a5 = Disable/Enable<br>•<br>0 = Disable<br>•<br>1 = Enable|a5 = Disable/Enable<br>•<br>0 = Disable<br>•<br>1 = Enable|
|a4 = Stop Bits<br>•<br>0 = 2 Bits<br>•<br>1 = 1 Bit|a4 = Stop Bits<br>•<br>0 = 2 Bits<br>•<br>1 = 1 Bit|
|a3 = Data Bits<br>•<br>0 = 7 Bits<br>•<br>1 = 8 Bits|a3 = Data Bits<br>•<br>0 = 7 Bits<br>•<br>1 = 8 Bits|


**String 2**



ZPL Commands


```
<STX>mmm,n,o,p,q,r,s,t,uuuuuuuu,v,www<ETX><CR><LF>

```

`mmm` function settings [1]

`n` unused

`o` **head up** flag (1 `=` head in up position)

`p` **ribbon out** flag (1 `=` ribbon out)

`q` **thermal transfer mode** flag (1 `=` Thermal Transfer Mode selected)

`r` Print Mode


234


Values 4 to 5
are supported
only in firmware
version V60.14.x,
V50.14.x, V53.
15.x, or later.



ZPL Commands


`0 =` Rewind

`1 =` Peel-Off

`2 =` Tear-Off

`3 =` Cutter

`4 =` Applicator

`5 =` Delayed cut

`6 =` Linerless Peel

`7 =` Linerless Rewind

`8 =` Partial Cutter

`9 =` RFID

`K =` Kiosk

`S = A =` Kiosk CutStream



`s` print width mode

`t` **label waiting** flag (1 `=` label waiting in Peel-off Mode)

`uuuuuuuu` labels remaining in batch

`v` **format while printing** flag (always 1)

`www` number of graphic images stored in memory


1. This string specifies the printer’s media type, sensor profile status, and communication diagnostics status.
As in String 1, this is a three-digit decimal representation of an eight-bit binary number. First, convert the
decimal number to a binary number. These values are only supported on the ZE500, Xi4, RXi4, ZM400/
ZM600, and RZ400/RZ600 printers.

```
mmmm = m [7] m [6] m [5] m [4] m [3] m [2] m [1] m [0]

```

The eight-digit binary number is read according to this table:



|m7 = Media Type<br>• 0 = Die-Cut<br>• 1 = Continuous|m4 m3 m2 m1 = Unused<br>• 0 = Off<br>• 1 = On|
|---|---|
|m6 = Sensor Profile<br>•<br>0 = Off|m0 = Print Mode<br>•<br>0 = Direct Thermal<br>•<br>1 = Thermal Transfer|
|m5 = Communications Diagnostics<br>•<br>0 = Off<br>•<br>1 = On|m5 = Communications Diagnostics<br>•<br>0 = Off<br>•<br>1 = On|


**String 3**

```
<STX>xxxx,y<ETX><CR><LF>

```




235


ZPL Commands


y 0 (static RAM not installed)

1 (static RAM installed)


236