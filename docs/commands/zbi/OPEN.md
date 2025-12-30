# OPEN




ZBI Commands


This command is used to open a port for transmitting and receiving data.


**Format**
```
   OPEN #<CHANNEL>: NAME <PORT$>
```

**Parameters**
`<CHANNEL>` = a number to use as a handle to the port for all future communications

**Values**


0 to 9


**Default**


a port must be specified

`<PORT$> =` port name to open.

**Example**
This is an example of how to use the OPEN command:

```
   10 OPEN #1: NAME "ZPL"

```

The port being opened no longer allows data to pass directly into its buffer, it disconnects, and the
interpreter now controls the data flow.


Data already in the buffer stays in the buffer.


**Comments**
This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.


486