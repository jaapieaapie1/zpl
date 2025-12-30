# SLEEP




ZBI Commands


This command specifies the time that the interpreter pauses. This command could be sent to the printer
after sending a label format to be printed. The interpreter pauses in its processing for the amount of time
specified.


**Format**
```
   SLEEP <A>
```

**Parameters**
`<A>` = the time in seconds (0 to 500) the interpreter pauses.

**Example**
This is an example of how to use the `SLEEP` command:

```
   10
   SLEEP 450

```

**Comments**
If a timer is needed, use the `Event` system. The timer will allow for processing other items, where
`SLEEP` will stop execution of any ZBI commands for the specified `SLEEP` period.

This is a program command and must be preceded by a line number.

Calling `SLEEP` with `<A>` set to zero will force the ZBI task to yield to the rest of the system and
allow any pending tasks to run (e.g., pending ZPL commands). If there are no pending tasks, ZBI will
sleep for a minimum of 8 milliseconds.


533