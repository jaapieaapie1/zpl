# device.host_status



When this command is sent to the printer, the printer sends three data strings back. To avoid confusion, the
host prints each string on a separate line.

This command is similar to the `~HS` ZPL command.

The response for this SGD command does not include the `STX` at the beginning of each data line and does
not include the `ETX` at the end of each data line as found in the `~HS` response. Additionally, the first and
second response lines for the SGD command contain a `CR/LF` at the end of each line.


**NOTE:** When the command is sent, the printer will not send a response to the host if the printer is
in one of these conditions:


          - MEDIA OUT


          - RIBBON OUT


          - HEAD OPEN


          - REWINDER FULL


          - HEAD OVER-TEMPERATURE


**Getvar**


To return the current setting value:

```
       ! U1 getvar "device.host_status"

```

**Result**


Three strings, each on their own line.

```
          "aaa,b,c,dddd,eee,f,g,h,iii,j,k,l
          mmm,n,o,p,q,r,s,t,uuuuuuuu,v,www
          xxxx,y"

```

See definitions for String 1, String 2, and String 3 below.


704


SGD Printer Commands


**String 1**

```
 "aaa,b,c,dddd,eee,f,g,h,iii,j,k,l"

```

The nine-digit binary number is read according to this table:


`aaa` communication (interface) settings [a]

`b` paper out flag (1 `=` paper out)

`c` pause flag (1 `=` pause active)

`dddd` label length (value in number of dots)

`eee` number of formats in receive buffer

`f` buffer full flag (1 `=` receive buffer full)

`g` communications diagnostic mode flag (1 = diagnostic mode active)

`h` partial format flag (1 `=` partial format in progress)

a This str ~~i~~ ng specifies the printer’s baud rate, number of data bits, number of stop bits, parity
setting, and type of handshaking. This value is a three-digit decimal representation of an eight-bit
binary number. To evaluate this parameter, first convert the decimal number to a binary number.


705


SGD Printer Commands


**String 2**

```
 "mmm,n,o,p,q,r,s,t,uuuuuuuu,v,www"

```

`mmm` function settings [a]

`n` unused

`o` head up flag (1 = head in up position)

`p` ribbon out flag (1 = ribbon out)

`q` thermal transfer mode flag (1 = Thermal Transfer Mode selected)

`r` Print Mode


                  - `0` Rewind



Values 4 to 5 are only
supported in firmware
version V60.14.x,
V50.14.x, V53. 15.x, or
later.




- `1` Peel-Off

- `2` Tear-Off

- `3` Cutter

- `4` Applicator

- `5` Delayed cut

- `6` Reserved [b]

- `7` Reserved [b]

- `8` Reserved [a]

- `9` RFID



`s` print width mode

`t` label waiting flag (1 = label waiting in Peel-off Mode)

`uuuuuuuu` labels remaining in batch

`v` format while printing flag (always 1)

`www` number of graphic images stored in memory

a This string specifies the printer’s media type, sensor profile status, and communication
diagnostics status. As in String 1, this is a three-digit decimal representation of an eight-bit binary
number. First, convert the decimal number to a binary number.

b These values are only supported on the Xi4, RXi4, ZM400/ZM600, RZ400/RZ600, and ZT200
Series printers.


The eight-digit binary number is read according to this table:


706


|Col1|Col2|
|---|---|
|||
|||
|||


SGD Printer Commands


**String 3**
```
 "xxxx,y"

```

`XXXX` password (printers running Link-OS v5.3 or
earlier versions)


`0000` password. (printers running Link-OS 6 or later
versions)


`y`  - 0 (static RAM not installed)


                                 - 1 (static RAM installed)


707


SGD Printer Commands