# OUTBYTE



This command outputs a byte to a port.


**Format**
```
          OUTBYTE [<CHANNEL>:] <A>
          OUTBYTE [<CHANNEL>:] <A$>
```

**Parameters**
`<CHANNEL>` = sends the byte to this port. Default = 0.

`<A>` = This is a numeric expression.

**Values**
0 through 255. If it is not within that range, it is truncated.

`<A$>` = This is the string expression. The first character is used. In the case of a NULL string, 0 is
sent.


**Example**
This is an example of how to use the OUTBYTE command:

```
          LET A$="Hello"
          OUTBYTE A$

```

This would only print the H character to the console.

```
          OUTBYTE 4

```

This would print the control character `EOT` to the console. See an ASCII table for a list of the control
characters.


**Comments**
This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.


497