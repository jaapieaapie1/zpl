# ^CM




ZPL Commands


The `^CM` command allows you to reassign a letter designation to the printer’s memory devices. If a format
already exists, you can reassign the memory device to the corresponding letter without forcing, altering, or
recreating the format itself.


**Change Memory Letter Designation**


Using this command affects every subsequent command that refers to specific memory locations.

**Format:** `^CMa,b,c,d`

|Parameters|Details|
|---|---|
|`a =` memory alias for`B:`|**Values:** `B:`, `E:`,`R:`, `A:`, and`NONE`<br>**Default:** `B:`|
|`b =` memory alias for`E:`|**Values:** `B:`, `E:`, `R:`, `A:`, and`NONE`<br>**Default:** `E:`|
|`c =` memory alias for`R:`|**Values:** `B:`, `E:`, `R:`, `A:`, and`NONE`<br>**Default:** `R:`|
|`d =` memory alias for`A:`|**Values:** `B:`, `E:`, `R:`, `A:`, and`NONE`<br>**Default:** `A:`|
|`e =` multiple alias|**Values:** `M`, or no value<br>**Default:** no value<br>•<br>This parameter is supported on Xi4 and ZM400/ZM600 printers using<br>firmware V53.17.7Z or later.<br>•<br>This parameter is supported on G-Series printers using firmware<br>versions v56.17.7Z and v61.17.7Z or later.<br>•<br>This parameter is supported on printers using firmware V60.17.7Z or<br>later.|



**Comments:** Unless the `e` (multiple alias) parameter is used, when two or more parameters specify the
same letter designator, all letter designators are set to their default values.

It is recommended that after entering the `^CM` command, `^JUS` is entered to save changes to EEPROM.
Any duplicate parameters entered will reset the letter designations back to the default.


If any of the parameters are out of specification, the command is ignored.


**Example:** This example designates letter E: to point to the B: memory device, and the letter B: to point to
the E:memory device.

```
^XA
^CME,B,R,A
^JUS
^XZ

```

**Example:** This example designates that content sent to, or read from the B: or E: memory locations will be
sent to or read from the E: memory location.


160




ZPL Commands

```
^XA

^CME,E,R,A,M
^JUS
^XZ

```

**Example:** This example designates that content sent to, or read from the A: or E: memory locations will be
sent to or read from the E: memory location.

```
^XA

^CMB,E,R,E,M
^JUS
^XZ

```

**Example:** This example designates that content sent to, or read from the A:, B: or E: memory locations will
be sent to or read from the E: memory location.

```
^XA

^CME,E,R,E,M
^JUS
^XZ

```

**NOTE:** The last three examples are the only valid uses of the multiple alias parameter.


161