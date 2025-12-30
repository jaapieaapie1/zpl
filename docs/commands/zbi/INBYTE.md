# INBYTE




ZBI Commands


This command forces the interpreter to pause until data is available. Use the `DATAREADY` function to
determine if there is data on the port.


**Format**
```
   INBYTE [<CHANNEL>:] <A>
   INBYTE [<CHANNEL>:] <A$>
```

`<A>` = integer value is set to the byte received.

**Parameters**
`<CHANNEL>` = reads from this port. Default = 0.

`<A$>` = A single byte string is created with the byte received.The first character is used. In the case
of a NULL string, 0 is sent.


**Example**
This is an example of how to use the INBYTE to create an echo program:

```
   10 INBYTE A$ !Takes one byte (char) from port #0
   20 PRINT A$ !Prints the character to the console
   30 GOTO 10

```

In this example, the interpreter pauses until the data is entered, then continues processing. This
command enters all bytes in a string or integer, including control codes.


**Comments**
`INBYTE` will block until a byte is received on the specified port. This can be an interactive
command that takes effect as soon as it is received by the printer, or a program command that is
preceded by a line number.


498