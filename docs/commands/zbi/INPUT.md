# INPUT




ZBI Commands


If the variable is numeric and the value entered cannot be converted to a number, it writes as 0. This
operation scans the data from left to right, shifting any number into the variable. It ignores any non-numeric
character except the return character, which terminates the input, or Ctrl-C ( `^C` ) which terminates the
program. The variable can be in string or numeric form.


**Format**
```
   INPUT [<CHANNEL>:] <A$> [,<B$>]*
   INPUT [<CHANNEL>:] <A>[,<B>]*
```

If the `[<channel>:]` is omitted, the default port, 0, will be used.

**Parameters**
`<CHANNEL>` = read data from this port. Default = 0.

`<A,B,...,N>` = variables to write.

When using multiple variables as targets, a corresponding number of lines are read. String and
numeric variables can be intermixed.


**Example**
This is an example of how to use the INPUT command:

```
   10 OPEN #1: NAME "ZPL"
   20 PRINT #1: "~HS"
   30 FOR I = 1 TO 3
   40 INPUT #1: A$
   50 PRINT A$
   60 NEXT I

```

In this example, a host status prints to the console after submitting the host status request `~HS` to
the ZPL port. The Input/Output command of the ZBI interpreter is limited to the communications
ports. File I/O is not supported.

`INPUT` ends processing a line with a `CR` or `LF` . This leads to a tricky situation. There are many ways
different systems end a line: `CR`, `CRLF`, `LF` . If the ZBI program only uses `INPUT`, the next execution
of the `INPUT` command will remove the extra `LF` or `CR`, in case of `LFCR` . However, if the program
instead uses `INBYTE`, `DATAREADY` or the other commands, the extra `LF` will show up on the port.
Here’s a simple workaround to explicitly look for the `CRLF` that is in use:
```
   SEARCHTO(<PORT>,CHR$(13)&CHR$(10),<INSTRING$>)

```

**NOTE:** Note: The `INPUT` command does not accept control characters or the delete
character. If these characters need to be processed, use the READ command.

**Comments**
This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.

If an invalid port is specified, `Error: Invalid port` is returned.


494


ZBI Commands


**Example**
This shows the input command reading in multiple lines.

```
 10 INPUT A$,B,C,D$,E$

```

Five lines would be read in: 3 strings and 2 numbers.


495