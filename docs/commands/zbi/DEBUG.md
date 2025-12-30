# DEBUG




ZBI Commands


`DEBUG` enables and disables the `TRACE` and `BREAK` commands.

**Format**
```
   DEBUG ON
   DEBUG OFF
```

**Parameters**
`ON` = turns the debug mode on enabling the `TRACE` and `BREAK` commands to be processed.

`OFF` = turns the debug mode off. This disables the `TRACE` mode and causes `BREAK` commands to
be ignored.


**Example**
N/A

**Comments**
This command has no effect on the processing of break points in ZBI-Developer. It is
recommended that you avoid using the `DEBUG` command when writing programs in the
environment, instead use the Debug capabilities of ZBI-Developer.
ZBI-Developer


461